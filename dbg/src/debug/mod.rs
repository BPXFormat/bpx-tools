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
use std::io::{Read, Seek};
use std::str::FromStr;
use crate::render::Render;

#[derive(Debug)]
pub enum Error<'a> {
    InvalidCommand {
        msg: &'static str,
        command: &'a str
    },
    Other(Box<dyn std::error::Error>)
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidCommand { msg, command } => write!(f, "invalid command {}: {}", command, msg),
            Error::Other(e) => write!(f, "{}", e)
        }
    }
}

impl<'a, T: std::error::Error + 'static> From<T> for Error<'a> {
    fn from(value: T) -> Self {
        Error::Other(Box::new(value))
    }
}

pub struct Command<'a> {
    pub cmd: &'a str,
    pub usage: &'a str,
    pub note: &'a str
}

pub trait Debugger {
    const TYPE_CODE: u8;
    fn available_commands(&self) -> &[Command];
    fn on_command<'a, R: Render>(&mut self, render: &mut R, cmd: &'a str, args: impl Iterator<Item = &'a str>) -> Result<(), Error<'a>>;
}

pub trait New<T: Read + Seek>: Debugger + Sized {
    fn new(container: bpx::core::Container<T>) -> Result<Self, Error<'static>>;
}

pub fn check_arg<'a, I: Iterator<Item = &'a str>, T: FromStr>(cmd: &'a str, args: &mut I) -> Result<T, Error<'a>> {
    let val = match args.next() {
        Some(v) => v,
        None => return Err(Error::InvalidCommand {
            command: cmd,
            msg: "Missing argument"
        })
    };
    val.parse().map_err(|_| Error::InvalidCommand {
        command: cmd,
        msg: "Parse error"
    })
}

macro_rules! impl_debugger {
    ({ $($name: ident),* }) => {
        impl<T: Read + Seek> Debugger for DynamicDebugger<T> {
            const TYPE_CODE: u8 = 0;

            fn available_commands(&self) -> &[Command] {
                match self {
                    $(
                        Self::$name(v) => v.available_commands()
                    )*
                }
            }

            fn on_command<'a, R: Render>(&mut self, render: &mut R, cmd: &'a str, args: impl Iterator<Item = &'a str>) -> Result<(), Error<'a>> {
                match self {
                    $(
                        Self::$name(v) => v.on_command(render, cmd, args),
                    )*
                }
            }
        }
    }
}

include!(env!("SRC_DEBUGGER_REGISTRY"));
