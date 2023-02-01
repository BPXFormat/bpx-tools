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

use std::fmt::{Debug, Display};
use std::io::Write;
use std::rc::Rc;
use bpx::sd::formatting::{Format, IndentType};
use bpx::sd::Value;
use bpxdbg::render;
use bpxdbg::render::ContentType;

pub struct Render;
pub struct Item;
pub struct Table {
    columns: Rc<Vec<usize>>
}
pub struct Row {
    columns: Rc<Vec<usize>>,
    row_id: usize
}
pub enum RawStream {
    Stdout,
    Null
}

impl render::Row for Row {
    fn value<T: Display>(&mut self, val: T) -> &mut Self {
        if self.row_id < self.columns.len() {
            print!("| {: <width$} |", val, width=self.columns[self.row_id] - 2);
        }
        self.row_id += 1;
        self
    }

    fn valued<T: Debug>(&mut self, val: T) -> &mut Self {
        if self.row_id < self.columns.len() {
            print!("| {: <width$?} |", val, width=self.columns[self.row_id] - 2);
        }
        self.row_id += 1;
        self
    }
}

impl render::Table for Table {
    type Row = Row;

    fn col<T: AsRef<str>>(&mut self, name: T, length: usize) -> &mut Self {
        if let Some(ptr) = Rc::get_mut(&mut self.columns) {
            ptr.push(length);
            print!("┌{:─^width$}┐", name.as_ref(), width=length);
        }
        self
    }

    fn row(&mut self) -> Self::Row {
        println!();
        Row {
            columns: self.columns.clone(),
            row_id: 0
        }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        println!();
        for v in &*self.columns {
            print!("└{:─^width$}┘", "", width=v);
        }
        println!();
    }
}

impl render::Item for Item {
    fn value<T: Display>(&mut self, val: T) -> &mut Self {
        print!(" {}", val);
        self
    }

    fn valued<T: Debug>(&mut self, val: T) -> &mut Self {
        print!(" {:?}", val);
        self
    }

    fn description<D: AsRef<str>>(&mut self, description: D) -> &mut Self {
        print!(" ({})", description.as_ref());
        self
    }

    fn note<N: AsRef<str>>(&mut self, note: N) -> &mut Self {
        print!(": {}", note.as_ref());
        self
    }
}

impl Drop for Item {
    fn drop(&mut self) {
        println!()
    }
}

impl render::List for Render {
    type Item = Item;

    fn item(&mut self) -> Self::Item {
        print!("  *");
        Item
    }
}

impl render::Group for Render {
    fn value<N: AsRef<str>, T: Display>(&mut self, name: N, val: T) -> &mut Self {
        println!("  {}: {}", name.as_ref(), val);
        self
    }

    fn valued<N: AsRef<str>, T: Debug>(&mut self, name: N, val: T) -> &mut Self {
        println!("  {}: {:?}", name.as_ref(), val);
        self
    }

    fn bpxsd<N: AsRef<str>>(&mut self, name: N, value: &Value) -> &mut Self {
        println!("  {}:", name.as_ref());
        match value.as_object() {
            None => println!("No BPXSD object, is the version of BPX really supported?"),
            Some(v) => println!("{}", v.format(IndentType::Spaces, 4))
        }
        self
    }
}

impl Write for RawStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            RawStream::Stdout => std::io::stdout().write(buf),
            RawStream::Null => Ok(buf.len())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            RawStream::Stdout => std::io::stdout().flush(),
            RawStream::Null => Ok(())
        }
    }
}

impl render::Render for Render {
    type Table = Table;
    type Group = Render;
    type List = Render;
    type RawStream = RawStream;

    fn table<T: AsRef<str>>(&mut self, name: T) -> Self::Table {
        println!("{}:", name.as_ref());
        Table {
            columns: Rc::new(Vec::new())
        }
    }

    fn group<T: AsRef<str>>(&mut self, name: T) -> Self::Group {
        println!("{}:", name.as_ref());
        Self
    }

    fn list<T: AsRef<str>>(&mut self, name: T, length: usize) -> Self::List {
        if length > 0 {
            println!("{} ({} item(s)):", name.as_ref(), length);
        } else {
            println!("{}:", name.as_ref());
        }
        Self
    }

    fn text<T: AsRef<str>>(&mut self, text: T) {
        println!("{}", text.as_ref());
    }

    fn bpxsd(&mut self, value: &Value) {
        match value.as_object() {
            None => println!("No BPXSD object, is the version of BPX really supported?"),
            Some(v) => println!("{}", v.format(IndentType::Spaces, 4))
        }
    }

    fn raw<T: AsRef<str>>(&mut self, name: T, content_type: ContentType) -> Self::RawStream {
        if content_type == ContentType::Text {
            println!("{}: {:?} >", name.as_ref(), content_type);
            RawStream::Stdout
        } else {
            println!("{}: {:?} (no preview available)", name.as_ref(), content_type);
            RawStream::Null
        }
    }
}
