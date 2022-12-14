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

use std::{io::Write, string::String};

use bpx::sd::{Array, DebugSymbols, Object, Value};

use crate::error::Result;

fn gen_layer_prefix(layer: usize) -> String
{
    let mut res = String::new();

    for _ in 0..layer {
        res.push('\t');
    }
    res
}

fn print_value<TWrite: Write>(layer: usize, value: &Value, out: &mut TWrite) -> Result<()>
{
    match value {
        Value::Null => writeln!(out, "NULL")?,
        Value::Uint8(v) => writeln!(out, "(Uint8) {}", v)?,
        Value::Uint16(v) => writeln!(out, "(Uint16) {}", v)?,
        Value::Uint32(v) => writeln!(out, "(Uint32) {}", v)?,
        Value::Uint64(v) => writeln!(out, "(Uint64) {}", v)?,
        Value::Int8(v) => writeln!(out, "(Int8) {}", v)?,
        Value::Int16(v) => writeln!(out, "(Int16) {}", v)?,
        Value::Int32(v) => writeln!(out, "(Int32) {}", v)?,
        Value::Int64(v) => writeln!(out, "(Int64) {}", v)?,
        Value::Float(v) => writeln!(out, "(Float) {}", v)?,
        Value::Double(v) => writeln!(out, "(Double) {}", v)?,
        Value::String(v) => writeln!(out, "{}", v)?,
        Value::Bool(v) => {
            if *v {
                writeln!(out, "true")?;
            } else {
                writeln!(out, "false")?;
            }
        },
        Value::Object(v) => print_object(layer + 1, v, out)?,
        Value::Array(v) => print_array(layer + 1, v, out)?
    }
    Ok(())
}

fn print_array<TWrite: Write>(layer: usize, array: &Array, out: &mut TWrite) -> Result<()>
{
    writeln!(out, "[")?;
    for i in 0..array.len() {
        print_value(layer, &array[i], out)?;
    }
    writeln!(out, "{}]", gen_layer_prefix(layer - 1))?;
    Ok(())
}

pub fn print_object<TWrite: Write>(layer: usize, object: &Object, out: &mut TWrite) -> Result<()>
{
    let prefix = gen_layer_prefix(layer);
    let debugger = match DebugSymbols::read(object) {
        Err(e) => {
            eprintln!("Warning: failed to read Object debug layer ({})", e);
            None
        },
        Ok(v) => Some(v)
    };

    writeln!(out, "{{")?;
    for key in object.get_keys() {
        match &debugger {
            None => write!(out, "{} {}", prefix, key)?,
            Some(d) => match d.lookup(*key) {
                None => write!(out, "{} {}", prefix, key)?,
                Some(name) => write!(out, "{} {}", prefix, name)?
            }
        };
        print_value(layer, &object[*key], out)?;
    }
    println!("{}}}", gen_layer_prefix(layer - 1));
    Ok(())
}
