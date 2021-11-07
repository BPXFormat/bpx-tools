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

use std::{
    borrow::Cow,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf}
};

use bpx::{
    decoder::IoBackend,
    variant::package::{utils::unpack_file, PackageDecoder}
};

use crate::error::UnpackError;

fn custom_unpack<TBackend: IoBackend>(
    package: &mut PackageDecoder<TBackend>,
    target: &Path,
    verbose: bool
) -> Result<(), UnpackError>
{
    let mut unnamed_count = 0;
    let (items, mut names) = package.read_object_table()?;
    for v in &items {
        let mut path: Cow<str> = names.load(v)?.into();
        if path.is_empty() {
            unnamed_count += 1;
            path = format!("unnamed_file_{}", unnamed_count).into();
        }
        if verbose {
            println!("Unpacking object name {} with {} byte(s)...", path, v.size);
        }
        let dest: PathBuf = [target, Path::new(path.as_ref())].iter().collect();
        if let Some(v) = dest.parent() {
            std::fs::create_dir_all(v)?;
        }
        unpack_file(package, v, &dest)?;
    }
    Ok(())
}

pub fn run(file: &Path, verbose: bool) -> Result<(), UnpackError>
{
    let mut decoder = PackageDecoder::new(BufReader::new(File::open(file)?))?;

    custom_unpack(&mut decoder, Path::new("."), verbose)?;
    Ok(())
}
