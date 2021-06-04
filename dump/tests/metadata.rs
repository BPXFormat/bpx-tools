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

use assert_cmd::Command;

const EXPECTED_OUTPUT: &'static str = "====> BPX Main Header <====
Type: P
Version: 1
File size: 1632
Number of sections: 2
====> End <====

====> BPX TypeExt <====
Architecture: Any
Platform: Any
Generator: BD
====> End <====

====> BPX Section Header Table <====
Section #0:
	Type: 255
	Size (after compression): 15
	Size: 15
	Flags:  CheckWeak
Section #1:
	Type: 1
	Size (after compression): 1529
	Size: 1529
	Flags:  CheckWeak
====> End <====

";

const EXPECTED_OUTPUT_HEX: &'static str = "====> BPX Main Header <====
Type: P
Version: 1
File size: 1632
Number of sections: 2
====> End <====

====> BPX TypeExt <====
04 04 42 44 00 00 00 00 00 00 00 00 00 00 00 00 
====> End <====

====> BPX Section Header Table <====
Section #0:
	Type: 255
	Size (after compression): 15
	Size: 15
	Flags:  CheckWeak
Section #1:
	Type: 1
	Size (after compression): 1529
	Size: 1529
	Flags:  CheckWeak
====> End <====

";

#[test]
fn dump_metadata_1()
{
    let assert = Command::cargo_bin("bpxdump")
        .unwrap()
        .args(&["-f", "tests/test.bpx", "-sm"])
        .assert();
    assert.success().stdout(EXPECTED_OUTPUT).stderr("");
}

#[test]
fn dump_metadata_2()
{
    let assert = Command::cargo_bin("bpxdump")
        .unwrap()
        .args(&["-f", "tests/test.bpx", "--sht", "--metadata"])
        .assert();
    assert.success().stdout(EXPECTED_OUTPUT).stderr("");
}

#[test]
fn dump_metadata_hex_1()
{
    let assert = Command::cargo_bin("bpxdump")
        .unwrap()
        .args(&["-f", "tests/test.bpx", "-smx"])
        .assert();
    assert.success().stdout(EXPECTED_OUTPUT_HEX).stderr("");
}

#[test]
fn dump_metadata_hex_2()
{
    let assert = Command::cargo_bin("bpxdump")
        .unwrap()
        .args(&["-f", "tests/test.bpx", "--sht", "--metadata", "--hex"])
        .assert();
    assert.success().stdout(EXPECTED_OUTPUT_HEX).stderr("");
}
