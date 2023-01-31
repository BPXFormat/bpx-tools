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
use bpx::sd::Value;

pub trait Row {
    fn value<T: Display>(&mut self, val: T) -> &mut Self;
    fn valued<T: Debug>(&mut self, val: T) -> &mut Self;
}

pub trait Table {
    type Row: Row;

    fn col<T: AsRef<str>>(&mut self, name: T, length: usize) -> &mut Self;
    fn row(&mut self) -> Self::Row;
}

pub trait Group {
    fn value<N: AsRef<str>, T: Display>(&mut self, name: N, val: T) -> &mut Self;
    fn valued<N: AsRef<str>, T: Debug>(&mut self, name: N, val: T) -> &mut Self;
    fn bpxsd<N: AsRef<str>>(&mut self, name: N, value: &Value) -> &mut Self;
}

pub trait Item {
    fn value<T: Display>(&mut self, val: T) -> &mut Self;
    fn valued<T: Debug>(&mut self, val: T) -> &mut Self;
    fn description<D: AsRef<str>>(&mut self, description: D) -> &mut Self;
    fn note<N: AsRef<str>>(&mut self, note: N) -> &mut Self;
}

pub trait List {
    type Item: Item;

    fn item(&mut self) -> Self::Item;
}

pub trait Render {
    type Table: Table;
    type Group: Group;
    type List: List;

    fn table<T: AsRef<str>>(&mut self, name: T) -> Self::Table;
    fn group<T: AsRef<str>>(&mut self, name: T) -> Self::Group;
    fn list<T: AsRef<str>>(&mut self, name: T, length: usize) -> Self::List;
    fn text<T: AsRef<str>>(&mut self, text: T);
    fn bpxsd(&mut self, value: &Value);
}
