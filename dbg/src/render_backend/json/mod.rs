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
use bpx::sd::Value;
use bpxdbg::render::{ContentType};
use bpxdbg::render;
use json::{JsonValue, object};
use self::bpxsd::ValueExt;

mod bpxsd;

pub struct Render;
pub struct List(JsonValue);
pub struct Item<'a> {
    items: &'a mut JsonValue,
    item: JsonValue
}
pub struct Table(JsonValue);
pub struct Row<'a>(&'a mut JsonValue);
pub struct Group(JsonValue);
pub struct NullStream;

impl Write for NullStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> render::Row for Row<'a> {
    fn value<T: Display>(&mut self, val: T) -> &mut Self {
        self.0.push(format!("{}", val)).unwrap();
        self
    }

    fn valued<T: Debug>(&mut self, val: T) -> &mut Self {
        self.0.push(format!("{:?}", val)).unwrap();
        self
    }
}

impl render::Table for Table {
    type Row<'a> = Row<'a>;

    fn col<T: AsRef<str>>(&mut self, name: T, length: usize) -> &mut Self {
        let col = object! {
            name: name.as_ref(),
            length: length
        };
        self.0["columns"].push(col).unwrap();
        self
    }

    fn row(&mut self) -> Self::Row<'_> {
        Row(&mut self.0["rows"])
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

impl render::Group for Group {
    fn value<N: AsRef<str>, T: Display>(&mut self, name: N, val: T) -> &mut Self {
        self.0["items"].push(object! {
            name: name.as_ref(),
            value: format!("{}", val)
        }).unwrap();
        self
    }

    fn valued<N: AsRef<str>, T: Debug>(&mut self, name: N, val: T) -> &mut Self {
        self.0["items"].push(object! {
            name: name.as_ref(),
            value: format!("{:?}", val)
        }).unwrap();
        self
    }

    fn bpxsd<N: AsRef<str>>(&mut self, name: N, value: &Value) -> &mut Self {
        self.0["items"].push(object! {
            name: name.as_ref(),
            value: value.to_json()
        }).unwrap();
        self
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        println!("{}", self.0)
    }
}

impl<'a> render::Item for Item<'a> {
    fn value<T: Display>(&mut self, val: T) -> &mut Self {
        self.item["value"] = format!("{}", val).into();
        self
    }

    fn valued<T: Debug>(&mut self, val: T) -> &mut Self {
        self.item["value"] = format!("{:?}", val).into();
        self
    }

    fn description<D: AsRef<str>>(&mut self, description: D) -> &mut Self {
        self.item["description"] = description.as_ref().into();
        self
    }

    fn note<N: AsRef<str>>(&mut self, note: N) -> &mut Self {
        self.item["note"] = note.as_ref().into();
        self
    }
}

impl<'a> Drop for Item<'a> {
    fn drop(&mut self) {
        self.items.push(self.item.take()).unwrap();
    }
}

impl render::List for List {
    type Item<'a> = Item<'a>;

    fn item(&mut self) -> Self::Item<'_> {
        Item {
            items: &mut self.0["items"],
            item: JsonValue::new_object()
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

impl render::Render for Render {
    type Table = Table;
    type Group = Group;
    type List = List;
    type RawStream = NullStream;

    fn table<T: AsRef<str>>(&mut self, name: T) -> Self::Table {
        let val = object! {
            type: "table",
            name: name.as_ref(),
            columns: [],
            rows: []
        };
        Table(val)
    }

    fn group<T: AsRef<str>>(&mut self, name: T) -> Self::Group {
        let val = object! {
            type: "group",
            name: name.as_ref(),
            items: []
        };
        Group(val)
    }

    fn list<T: AsRef<str>>(&mut self, name: T, length: usize) -> Self::List {
        let val = object! {
            type: "list",
            name: name.as_ref(),
            length: length,
            items: []
        };
        List(val)
    }

    fn text<T: AsRef<str>>(&mut self, text: T) {
        let val = object! {
            type: "text",
            text: text.as_ref()
        };
        println!("{}", val);
    }

    fn bpxsd(&mut self, value: &Value) {
        println!("{}", value.to_json());
    }

    fn raw<T: AsRef<str>>(&mut self, name: T, content_type: ContentType) -> Self::RawStream {
        let val = object! {
            type: "stream",
            name: name.as_ref(),
            content: format!("{:?}", content_type),
            port: 0
        };
        println!("{}", val);
        //TODO: Implement upload to TcpStream.
        NullStream
    }
}
