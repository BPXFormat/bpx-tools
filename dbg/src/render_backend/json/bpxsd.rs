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

use bpx::sd::{Array, Object, Value};
use bpx::sd::debug::Debugger;
use json::{JsonValue, object};

pub trait ValueExt {
    fn to_json(&self) -> JsonValue;
}

impl ValueExt for Array {
    fn to_json(&self) -> JsonValue {
        let mut val = JsonValue::new_array();
        for v in self {
            val.push(v.to_json()).unwrap();
        }
        val
    }
}

impl ValueExt for Object {
    fn to_json(&self) -> JsonValue {
        match Debugger::attach(self) {
            Ok(dbg) => {
                let mut val = JsonValue::new_array();
                for (name, hash, value) in &dbg {
                    let item = object! {
                        name: name,
                        hash: format!("{}", hash.into_inner()),
                        value: value.to_json()
                    };
                    val.push(item).unwrap();
                }
                val
            },
            Err(e) => format!("invalid object: {}", e).into()
        }
    }
}

impl ValueExt for Value {
    fn to_json(&self) -> JsonValue {
        match self {
            Value::Null => object! { type: "null" },
            Value::Bool(v) => object! { type: "bool", value: format!("{}", v) },
            Value::Uint8(v) => object! { type: "u8", value: format!("{}", v) },
            Value::Uint16(v) => object! { type: "u16", value: format!("{}", v) },
            Value::Uint32(v) => object! { type: "u32", value: format!("{}", v) },
            Value::Uint64(v) => object! { type: "u64", value: format!("{}", v) },
            Value::Int8(v) => object! { type: "i8", value: format!("{}", v) },
            Value::Int16(v) => object! { type: "i16", value: format!("{}", v) },
            Value::Int32(v) => object! { type: "i32", value: format!("{}", v) },
            Value::Int64(v) => object! { type: "i64", value: format!("{}", v) },
            Value::Float(v) => object! { type: "f32", value: format!("{}", v) },
            Value::Double(v) => object! { type: "f64", value: format!("{}", v) },
            Value::String(v) => object! { type: "str", value: v.clone() },
            Value::Array(v) => object! { type: "array", value: v.to_json() },
            Value::Object(v) => object! { type: "object", value: v.to_json() }
        }
    }
}
