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

use std::fmt::{Display, Formatter};

use bpx::macros::impl_err_conversion;

pub enum Error
{
    Bpx(bpx::core::error::ReadError),
    Io(std::io::Error),
    Sd(bpx::sd::error::ReadError),
    Parsing(String),
    SectionNotFound(u32),
    BinaryOutput
}

impl_err_conversion!(
    Error {
        bpx::core::error::ReadError => Bpx,
        std::io::Error => Io,
        bpx::sd::error::ReadError => Sd
    }
);

impl Display for Error
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self {
            Error::Bpx(e) => write!(f, "BPX error: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Sd(e) => write!(f, "BPXSD error: {}", e),
            Error::Parsing(s) => write!(f, "Could not parse value ({})", s),
            Error::SectionNotFound(id) => write!(f, "Could not find section with index {}", id),
            Error::BinaryOutput => f.write_str("Outputing binary data to standard output can mess-up your terminal, please use --force if you're sure to continue")
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
