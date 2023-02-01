// Copyright (c) 2023, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use bpx::core::Container;
use crate::debug::{Debugger, DynamicDebugger};
use crate::render::{Item, List, Render};

#[derive(Debug)]
pub enum Error<'a> {
    UnknownTypeCode(u8),
    Io(std::io::Error),
    Bpx(bpx::core::error::Error),
    Debugger(crate::debug::Error<'a>)
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "failed to open file: {}", e),
            Error::Bpx(e) => write!(f, "failed to open BPX container: {}", e),
            Error::Debugger(e) => write!(f, "debugger error: {}", e),
            Error::UnknownTypeCode(code) => {
                match char::try_from(*code as u32) {
                    Ok(v) => write!(f, "no debugger found for BPX type {}", v),
                    Err(_) => write!(f, "no debugger found for BPX type code 0x{:X}", code)
                }
            }
        }
    }
}

impl<'a> std::error::Error for Error<'a> { }

pub struct Runtime<R> {
    debugger: DynamicDebugger<BufReader<File>>,
    render: R
}

impl<R: Render> Runtime<R> {
    pub fn new(path: &Path, render: R) -> Result<Runtime<R>, Error<'static>> {
        let file = File::open(path).map_err(Error::Io)?;
        let container = Container::open(BufReader::new(file))
            .map_err(Error::Bpx)?;
        let fuckingrust = container.main_header().ty;
        let debugger = DynamicDebugger::from_type_code(fuckingrust, container)
            .ok_or_else(|| Error::UnknownTypeCode(fuckingrust))?.map_err(Error::Debugger)?;
        Ok(Runtime {
            debugger,
            render
        })
    }

    pub fn run<'a>(&mut self, command_line: &'a str) -> Result<(), Error<'a>> {
        let mut args = command_line.split(" ");
        if let Some(cmd) = args.next() {
            let commands = self.debugger.available_commands();
            match cmd {
                "help" => {
                    let mut list = self.render.list("Commands", commands.len());
                    for cmd in commands {
                        list.item().value(cmd.cmd).description(cmd.usage).note(cmd.note);
                    }
                },
                _ => {
                    if commands.iter().any(|v| v.cmd == cmd) {
                        self.debugger.on_command(&mut self.render, cmd, args).map_err(Error::Debugger)?;
                    } else {
                        self.render.text(format!("Unknown command '{}', type 'help' for help", cmd));
                    }
                }
            }
        }
        Ok(())
    }
}
