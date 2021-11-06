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

const EXPECTED_OUTPUT_HEX: &str = "====> BPX Main Header <====
Type: P
Version: 1
File size: 1632
Number of sections: 2
====> End <====

4C 49 43 45 4E 53 45 5F 4C 46 2E 74 78 74 00 
";

const EXPECTED_OUTPUT_RAW: &str = "====> BPX Main Header <====
Type: P
Version: 1
File size: 1632
Number of sections: 2
====> End <====

";

const EXPECTED_ERROR_RAW: &str =
    "Outputing binary data to standard output can mess-up your terminal, please use --force if you're sure to continue
";

#[test]
fn dump_section_hex_1()
{
    let assert = Command::cargo_bin("bpxdump")
        .unwrap()
        .args(&["-f", "tests/test.bpx", "-xd", "0"])
        .assert();
    assert.success().stdout(EXPECTED_OUTPUT_HEX).stderr("");
}

#[test]
fn dump_section_hex_2()
{
    let assert = Command::cargo_bin("bpxdump")
        .unwrap()
        .args(&["-f", "tests/test.bpx", "--dump", "0", "--hex"])
        .assert();
    assert.success().stdout(EXPECTED_OUTPUT_HEX).stderr("");
}

#[test]
fn dump_section_raw_1()
{
    let assert = Command::cargo_bin("bpxdump")
        .unwrap()
        .args(&["-f", "tests/test.bpx", "-d", "0"])
        .assert();
    assert
        .failure()
        .stderr(EXPECTED_ERROR_RAW)
        .stdout(EXPECTED_OUTPUT_RAW);
}

#[test]
fn dump_section_raw_2()
{
    let assert = Command::cargo_bin("bpxdump")
        .unwrap()
        .args(&["-f", "tests/test.bpx", "--dump", "0"])
        .assert();
    assert
        .failure()
        .stderr(EXPECTED_ERROR_RAW)
        .stdout(EXPECTED_OUTPUT_RAW);
}
