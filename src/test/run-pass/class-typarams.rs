// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// pretty-expanded FIXME #23616

use std::marker::PhantomData;

struct cat<U> {
    meows : uint,
    how_hungry : int,
    m: PhantomData<U>
}

impl<U> cat<U> {
    pub fn speak(&mut self) { self.meows += 1; }
    pub fn meow_count(&mut self) -> uint { self.meows }
}

fn cat<U>(in_x : uint, in_y : int) -> cat<U> {
    cat {
        meows: in_x,
        how_hungry: in_y,
        m: PhantomData
    }
}


pub fn main() {
  let _nyan : cat<int> = cat::<int>(52, 99);
  //  let mut kitty = cat(1000, 2);
}
