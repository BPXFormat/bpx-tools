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

use std::path::Path;
use bp3d_util::result::ResultExt;
use clap::clap_app;

mod bpxinfo;
mod error;
//mod printsd;
mod type_ext_maps;

fn main()
{
    let matches = clap_app!(bpxdump =>
        (version: "2.0")
        (author: "BlockProject3D <https://github.com/BlockProject3D>")
        (about: "Dumps content of a given BPX file")
        (@arg file: -f --file +required +takes_value "Path to the BPX file to debug")
        (@arg sht: -s --sht "Prints the section header table (SHT)")
        (@arg metadata: -m --metadata "Prints metadata (metadata here refers to the TypeExt block)")
        (@arg hex: -x --hex "Prints data in hex")
        (@arg force: --force "Force prints data to terminal ignoring potential terminal destruction")
        (@arg section_id: -d --dump +takes_value "Dumps the content of the section identified by the given index")
        (@arg out_file: -o --output +takes_value "Save dump output to a file")
        (@arg bpxsd: --bpxsd "Parse the section to print (specified in -d) as a BPX Structured Data Object (BPXSD)")
        (@arg skip_signature: --skipsignature "Skip file signature verification")
        (@arg skip_version: --skipversion "Skip file version verification")
        (@arg skip_checksum: --skipchecksum "Skip checksum verifications")
    )
    .get_matches();
    let file = matches.value_of("file").unwrap();

    bpxinfo::run(Path::new(file), &matches).expect_exit("bpxdump", 1);
}
