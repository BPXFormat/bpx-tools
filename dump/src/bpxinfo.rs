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

use std::{fs::File, io::Write, path::Path, rc::Rc, string::String};
use std::io::BufReader;

use bpx::{
    decoder::{Decoder, IoBackend},
    section::AutoSection,
    Interface
};
use clap::ArgMatches;

use super::type_ext_maps::get_type_ext_map;
use crate::error::{Error, Result};

fn print_main_header<TInterface: Interface>(bpx: &TInterface)
{
    println!("====> BPX Main Header <====");
    println!("Type: {}", bpx.get_main_header().btype as char);
    println!("Version: {}", bpx.get_main_header().version);
    println!("File size: {}", bpx.get_main_header().file_size);
    println!("Number of sections: {}", bpx.get_main_header().section_num);
    println!("====> End <====");
    println!();
}

fn print_sht<TInterface: Interface>(bpx: &TInterface)
{
    println!("====> BPX Section Header Table <====");
    for i in 0..bpx.get_main_header().section_num {
        let section = bpx.get_section_header(bpx.find_section_by_index(i).unwrap());
        println!("Section #{}:", i);
        println!("\tType: {}", section.btype);
        println!("\tSize (after compression): {}", section.csize);
        println!("\tSize: {}", section.size);
        let mut flags = String::new();
        if section.flags & 0x1 == 0x1 {
            flags.push_str(" | CompressZlib");
        }
        if section.flags & 0x2 == 0x2 {
            flags.push_str(" | CompressXZ");
        }
        if section.flags & 0x4 == 0x4 {
            flags.push_str(" | CheckCrc32");
        }
        if section.flags & 0x8 == 0x8 {
            flags.push_str(" | CheckWeak");
        }
        if section.flags & 0x8 != 0x8 && section.flags & 0x4 != 0x4 {
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
    return Ok(());
}

fn print_metadata<TInterface: Interface>(bpx: &TInterface, hex: bool) -> Result<()>
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
    return Ok(());
}

fn print_section_hex<TWrite: Write>(section: &Rc<AutoSection>, out: &mut TWrite) -> Result<()>
{
    let mut rin = section.open()?;
    let mut buf: [u8; 8192] = [0; 8192];
    let mut res = rin.read(&mut buf)?;
    while res > 0 {
        hex_print(&buf[0..res], out)?;
        res = rin.read(&mut buf)?;
    }
    writeln!(out)?;
    return Ok(());
}

fn print_section_sd<TWrite: Write>(section: &Rc<AutoSection>, out: &mut TWrite) -> Result<()>
{
    let mut rin = section.open()?;
    let object = bpx::sd::Object::read(rin.as_mut())?;
    super::printsd::print_object(1, &object, out)?;
    return Ok(());
}

fn print_section_raw<TWrite: Write>(section: &Rc<AutoSection>, out: &mut TWrite) -> Result<()>
{
    let mut rin = section.open()?;
    let mut buf: [u8; 8192] = [0; 8192];
    let mut res = rin.read(&mut buf)?;
    while res > 0 {
        out.write_all(&buf[0..res])?;
        res = rin.read(&mut buf)?;
    }
    return Ok(());
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

fn open_section_print<TBackend: IoBackend, TWrite: Write>(
    bpx: &mut Decoder<TBackend>,
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
    let section = bpx.load_section(section)?;
    return match opts.format {
        PrintFormat::Hex => print_section_hex(section, &mut opts.output),
        PrintFormat::Sd => print_section_sd(section, &mut opts.output),
        PrintFormat::Raw => print_section_raw(section, &mut opts.output)
    };
}

pub fn run(file: &Path, matches: &ArgMatches) -> Result<()>
{
    let mut bpx = Decoder::new(BufReader::new(File::open(file)?))?;

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
    return Ok(());
}
