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

use std::io::{Read, Seek};
use bpx::core::Container;
use crate::debug::{Command, Debugger, Error, New};
use crate::render::{Group, Render, Row, Table};

pub struct Package<T> {
    package: bpx::package::Package<T>
}

impl<T: Read + Seek> Debugger for Package<T> {
    const TYPE_CODE: u8 = b'P';

    fn available_commands(&self) -> &[Command] {
        &[
            Command {
                cmd: "settings",
                usage: "settings",
                note: "Display package settings",
            },
            Command {
                cmd: "metadata",
                usage: "metadata",
                note: "Display package metadata if any",
            },
            Command {
                cmd: "objects",
                usage: "objects",
                note: "Display the object table",
            }
        ]
    }

    fn on_command<'a, R: Render>(&mut self, render: &mut R, cmd: &'a str, _: impl Iterator<Item = &'a str>) -> Result<(), Error<'a>> {
        match cmd {
            "settings" => {
                let settings = self.package.settings();
                render.group("Settings")
                    .valued("Architecture", settings.architecture)
                    .valued("Platform", settings.platform)
                    .valued("Type Code", settings.type_code);
                Ok(())
            },
            "metadata" => {
                let metadata = self.package.load_metadata()?;
                if metadata.is_null() {
                    render.text("No metadata found in package");
                } else {
                    render.bpxsd(metadata);
                }
                Ok(())
            },
            "objects" => {
                let objects = self.package.objects()?;
                let mut table = render.table("Objects");
                table.col("Name", 48)
                    .col("Start", 5)
                    .col("Offset", 10)
                    .col("Size", 10);
                for header in &objects {
                    table.row().value(objects.load_name(header)?)
                        .value(header.start).value(header.offset).value(header.size);
                }
                Ok(())
            }
            _ => unreachable!()
        }
    }
}

impl<T: Read + Seek> New<T> for Package<T> {
    fn new(container: Container<T>) -> Result<Self, Error<'static>> {
        Ok(Self {
            package: container.try_into()?
        })
    }
}
