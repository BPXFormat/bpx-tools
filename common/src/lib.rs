// Copyright (c) 2021, BlockProject 3D
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

use std::string::String;

/*pub fn error(err: &Error)
{
    match err
    {
        Error::BpxRead(e) => eprintln!("BPX error: {}", e),
        Error::Io(e) => eprintln!("IO error: {}", e),
        Error::Parsing(e) => eprintln!("Parsing error: {}", e),
        Error::SectionNotFound(v) => eprintln!("Could not find section with index {}", v),
        Error::BinaryOutput => eprintln!("Outputing binary data to standard output can mess-up your terminal, please use --force if you're sure to continue")
    };
    std::process::exit(1);
}*/

pub fn error(err: &Error)
{
    match err
    {
        //Error::BpxRead(e) => eprintln!("BPX error: {}", e),
        Error::Io(e) => eprintln!("IO error: {}", e),
        Error::Parsing(e) => eprintln!("Parsing error: {}", e),
        Error::SectionNotFound(v) => eprintln!("Could not find section with index {}", v),
        Error::BinaryOutput => eprintln!("Outputing binary data to standard output can mess-up your terminal, please use --force if you're sure to continue"),
        _ => eprintln!("Unknown")
    };
    std::process::exit(1);
}

pub enum Error
{
    BpxRead(bpx::error::ReadError),
    BpxWrite(bpx::error::WriteError),
    BpxpRead(bpx::variant::package::error::ReadError),
    BpxpWrite(bpx::variant::package::error::WriteError),
    Strings(bpx::strings::ReadError),
    Section(bpx::section::Error),
    SdRead(bpx::sd::error::ReadError),
    Io(std::io::Error),
    Parsing(String),
    SectionNotFound(u32),
    BinaryOutput
}

impl From<std::io::Error> for Error
{
    fn from(e: std::io::Error) -> Self
    {
        return Error::Io(e);
    }
}

impl From<bpx::error::ReadError> for Error
{
    fn from(e: bpx::error::ReadError) -> Self
    {
        return Error::BpxRead(e);
    }
}

impl From<bpx::error::WriteError> for Error
{
    fn from(e: bpx::error::WriteError) -> Self
    {
        return Error::BpxWrite(e);
    }
}

impl From<bpx::variant::package::error::ReadError> for Error
{
    fn from(e: bpx::variant::package::error::ReadError) -> Self
    {
        return Error::BpxpRead(e);
    }
}

impl From<bpx::variant::package::error::WriteError> for Error
{
    fn from(e: bpx::variant::package::error::WriteError) -> Self
    {
        return Error::BpxpWrite(e);
    }
}

impl From<bpx::strings::ReadError> for Error
{
    fn from(e: bpx::strings::ReadError) -> Self
    {
        return Error::Strings(e);
    }
}

impl From<bpx::section::Error> for Error
{
    fn from(e: bpx::section::Error) -> Self
    {
        return Error::Section(e);
    }
}

impl From<bpx::sd::error::ReadError> for Error
{
    fn from(e: bpx::sd::error::ReadError) -> Self
    {
        return Error::SdRead(e);
    }
}

pub type Result<T> = std::result::Result<T, Error>;
