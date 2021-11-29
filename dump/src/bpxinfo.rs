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
    fs::File,
    io::{BufReader, Write},
    path::Path,
    string::String
};
use std::io::{Read, Seek};
use bpx::core::{Container, SectionMut};
use bpx::core::header::{FLAG_CHECK_CRC32, FLAG_CHECK_WEAK, FLAG_COMPRESS_XZ, FLAG_COMPRESS_ZLIB};

use clap::ArgMatches;

use super::type_ext_maps::get_type_ext_map;
use crate::error::{Error, Result};

fn print_main_header<T>(bpx: &Container<T>)
{
    println!("====> BPX Main Header <====");
    println!("Type: {}", bpx.get_main_header().btype as char);
    println!("Version: {}", bpx.get_main_header().version);
    println!("File size: {}", bpx.get_main_header().file_size);
    println!("Number of sections: {}", bpx.get_main_header().section_num);
    println!("====> End <====");
    println!();
}

fn print_sht<T>(bpx: &Container<T>)
{
    println!("====> BPX Section Header Table <====");
    for v in bpx.iter() {
        println!("Section #{}:", v.index());
        println!("\tType: {}", v.header().btype);
        println!("\tSize (after compression): {}", v.header().csize);
        println!("\tSize: {}", v.header().size);
        let mut flags = String::new();
        if v.header().flags & FLAG_COMPRESS_ZLIB == FLAG_COMPRESS_ZLIB {
            flags.push_str(" | CompressZlib");
        }
        if v.header().flags & FLAG_COMPRESS_XZ == FLAG_COMPRESS_XZ {
            flags.push_str(" | CompressXZ");
        }
        if v.header().flags & FLAG_CHECK_CRC32 == FLAG_CHECK_CRC32 {
            flags.push_str(" | CheckCrc32");
        }
        if v.header().flags & FLAG_CHECK_WEAK == FLAG_CHECK_WEAK {
            flags.push_str(" | CheckWeak");
        }
        if v.header().flags & FLAG_CHECK_WEAK != FLAG_CHECK_WEAK && v.header().flags & FLAG_CHECK_CRC32 != FLAG_CHECK_CRC32 {
            flags.push_str(" | CheckNone");
        }
        println!("\tFlags: {}", &flags[2..]);
    }
    println!("====> End <====");
    println!();
}

fn hex_print<TWrite: Write>(block: &[u8], output: &mut TWrite) -> Result<()>
{
    for (i, byte) in block.iter().enumerate() {
        if i != 0 && i % 16 == 0 {
            writeln!(output)?;
        }
        write!(output, "{:02X} ", byte)?;
    }
    Ok(())
}

fn print_metadata<T>(bpx: &Container<T>, hex: bool) -> Result<()>
{
    println!("====> BPX TypeExt <====");
    if hex {
        hex_print(&bpx.get_main_header().type_ext, &mut std::io::stdout())?;
        println!();
    } else {
        match get_type_ext_map(bpx.get_main_header().btype) {
            Some(func) => func(&bpx.get_main_header().type_ext),
            None => {
                hex_print(&bpx.get_main_header().type_ext, &mut std::io::stdout())?;
                println!();
            }
        }
    }
    println!("====> End <====");
    println!();
    Ok(())
}

fn print_section_hex<T: Read + Seek, TWrite: Write>(mut section: SectionMut<T>, out: &mut TWrite) -> Result<()>
{
    let rin = section.load()?;
    let mut buf: [u8; 8192] = [0; 8192];
    let mut res = rin.read(&mut buf)?;
    while res > 0 {
        hex_print(&buf[0..res], out)?;
        res = rin.read(&mut buf)?;
    }
    writeln!(out)?;
    Ok(())
}

fn print_section_sd<T: Read + Seek, TWrite: Write>(mut section: SectionMut<T>, out: &mut TWrite) -> Result<()>
{
    let rin = section.load()?;
    let object = bpx::sd::Object::read(rin)?;
    super::printsd::print_object(1, &object, out)?;
    Ok(())
}

fn print_section_raw<T: Read + Seek, TWrite: Write>(mut section: SectionMut<T>, out: &mut TWrite) -> Result<()>
{
    let rin = section.load()?;
    let mut buf: [u8; 8192] = [0; 8192];
    let mut res = rin.read(&mut buf)?;
    while res > 0 {
        out.write_all(&buf[0..res])?;
        res = rin.read(&mut buf)?;
    }
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum PrintFormat
{
    Hex,
    Sd,
    Raw
}

struct PrintOptions<'a, TWrite: Write>
{
    section_id_str: &'a str,
    output: TWrite,
    format: PrintFormat
}

fn open_section_print<T: Read + Seek, TWrite: Write>(
    bpx: &mut Container<T>,
    mut opts: PrintOptions<TWrite>
) -> Result<()>
{
    let section_id: u32 = match opts.section_id_str.parse() {
        Ok(id) => id,
        Err(e) => {
            return Err(Error::Parsing(format!(
                "Could not parse section index {} ({})",
                opts.section_id_str, e
            )));
        }
    };
    let section = match bpx.find_section_by_index(section_id) {
        Some(section) => section,
        None => return Err(Error::SectionNotFound(section_id))
    };
    let section = bpx.get_mut(section);
    match opts.format {
        PrintFormat::Hex => print_section_hex(section, &mut opts.output),
        PrintFormat::Sd => print_section_sd(section, &mut opts.output),
        PrintFormat::Raw => print_section_raw(section, &mut opts.output)
    }
}

pub fn run(file: &Path, matches: &ArgMatches) -> Result<()>
{
    let mut bpx = Container::open(BufReader::new(File::open(file)?))?;

    print_main_header(&bpx);
    if matches.is_present("metadata") {
        print_metadata(&bpx, matches.is_present("hex"))?;
    }
    if matches.is_present("sht") {
        print_sht(&bpx);
    }
    if let Some(section_id_str) = matches.value_of("section_id") {
        let format = {
            if matches.is_present("bpxsd") {
                PrintFormat::Sd
            } else if matches.is_present("hex") {
                PrintFormat::Hex
            } else {
                PrintFormat::Raw
            }
        };
        if format == PrintFormat::Raw && !matches.is_present("force") {
            return Err(Error::BinaryOutput);
        }
        match matches.value_of("out_file") {
            None => {
                open_section_print(
                    &mut bpx,
                    PrintOptions {
                        format,
                        section_id_str,
                        output: std::io::stdout()
                    }
                )?;
            },
            Some(s) => {
                open_section_print(
                    &mut bpx,
                    PrintOptions {
                        format,
                        section_id_str,
                        output: File::create(s)?
                    }
                )?;
            }
        }
    }
    Ok(())
}
