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

pub enum UnpackError
{
    Bpxp(bpx::variant::package::error::ReadError),
    Io(std::io::Error),
    Strings(bpx::strings::ReadError)
}

impl_err_conversion!(
    UnpackError {
        bpx::variant::package::error::ReadError => Bpxp,
        std::io::Error => Io,
        bpx::strings::ReadError => Strings
    }
);

pub enum PackError
{
    Bpxp(bpx::variant::package::error::WriteError),
    Bpx(bpx::error::WriteError),
    Io(std::io::Error)
}

impl_err_conversion!(
    PackError {
        bpx::variant::package::error::WriteError => Bpxp,
        bpx::error::WriteError => Bpx,
        std::io::Error => Io
    }
);

impl Display for PackError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self {
            PackError::Bpxp(e) => f.write_fmt(format_args!("BPXP error: {}", e)),
            PackError::Bpx(e) => f.write_fmt(format_args!("BPX error: {}", e)),
            PackError::Io(e) => f.write_fmt(format_args!("IO error: {}", e))
        }
    }
}

impl Display for UnpackError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self {
            UnpackError::Bpxp(e) => f.write_fmt(format_args!("BPXP error: {}", e)),
            UnpackError::Io(e) => f.write_fmt(format_args!("IO error: {}", e)),
            UnpackError::Strings(e) => f.write_fmt(format_args!("Strings error: {}", e))
        }
    }
}
