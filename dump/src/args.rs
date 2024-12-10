// Copyright (c) 2024, BlockProject 3D
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

use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, ValueEnum, Eq, PartialEq)]
pub enum FileOption {
    SkipSignature,
    SkipVersion,
    SkipChecksum
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the BPX file to open.
    #[arg(short='f')]
    pub file: PathBuf,

    /// Prints the section header table (SHT).
    #[arg(short='t', long="table")]
    pub sht: bool,

    /// Prints metadata (metadata here refers to the TypeExt block).
    #[arg(long="metadata", short='m')]
    pub metadata: bool,

    /// Prints data in hex.
    #[arg(long="hex", short='x')]
    pub hex: bool,

    /// Force prints data to terminal ignoring potential terminal destruction.
    #[arg(long="force")]
    pub force: bool,

    /// Dumps the content of the section identified by the given index.
    #[arg(short='d', long="dump")]
    pub section: Option<u32>,

    /// Save dump output to a file.
    #[arg(long="output", short='o')]
    pub output: Option<PathBuf>,

    /// Parse the section to print (specified in -d) as a BPX Structured Data Object (BPXSD).
    #[arg(long="bpxsd", short='s')]
    pub bpxsd: bool,

    /// Specifies the offset at which to start the decoding of the section to dump.
    #[arg(long="offset")]
    pub offset: Option<u32>,

    /// Specifies the options to use when opening the BPX file.
    #[arg(long="option", short='O')]
    pub options: Vec<FileOption>
}
