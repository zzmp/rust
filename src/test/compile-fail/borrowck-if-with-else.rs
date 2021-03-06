// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate debug;

fn foo(x: int) { println!("{:?}", x); }

fn main() {
    let x: int;
    if 1 > 2 {
        println!("whoops");
    } else {
        x = 10;
    }
    foo(x); //~ ERROR use of possibly uninitialized variable: `x`
}
