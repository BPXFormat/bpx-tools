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

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::io::Write;
use crate::basic_cli_render::BasicCliRender;

mod runtime;
mod debug;
mod render;
mod basic_cli_render;

fn print_prompt() {
    let mut lock = std::io::stdout().lock();
    write!(lock, "> ").unwrap();
    lock.flush().unwrap();
}

fn run(mut runtime: runtime::Runtime<BasicCliRender>) -> std::io::Result<()> {
    let lines = BufReader::new(std::io::stdin()).lines();
    print_prompt();
    for line in lines {
        let line = line?;
        if let Err(e) = runtime.run(&line) {
            println!("failed to run '{}': {}", line, e);
        }
        print_prompt();
    }
    Ok(())
}

fn main() {
    let mut args = std::env::args_os();
    if args.len() != 2 {
        eprintln!("USAGE: {:?} <path to executable file>", args.next());
        std::process::exit(1);
    }
    args.next();
    let path = args.next().map(PathBuf::from).unwrap();
    let runtime = match runtime::Runtime::new(&path, BasicCliRender) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("couldn't load target: {}", e);
            std::process::exit(1);
        }
    };
    if let Err(e) = run(runtime) {
        eprintln!("failed to read standard input: {}", e);
        std::process::exit(1);
    }
}
