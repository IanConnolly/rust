// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!
 * Implementations of things like `Eq` for fixed-length arrays
 * up to a certain length. Eventually we should able to generalize
 * to all lengths.
 */

#![stable]
#![experimental] // not yet reviewed

use cmp::*;
use option::{Option};

// macro for implementing n-ary tuple functions and operations
macro_rules! array_impls {
    ($($N:expr)+) => {
        $(
            #[unstable = "waiting for PartialEq to stabilize"]
            impl<T:PartialEq> PartialEq for [T, ..$N] {
                #[inline]
                fn eq(&self, other: &[T, ..$N]) -> bool {
                    self[] == other[]
                }
                #[inline]
                fn ne(&self, other: &[T, ..$N]) -> bool {
                    self[] != other[]
                }
            }

            #[unstable = "waiting for Eq to stabilize"]
            impl<T:Eq> Eq for [T, ..$N] { }

            #[unstable = "waiting for PartialOrd to stabilize"]
            impl<T:PartialOrd> PartialOrd for [T, ..$N] {
                #[inline]
                fn partial_cmp(&self, other: &[T, ..$N]) -> Option<Ordering> {
                    PartialOrd::partial_cmp(&self[], &other[])
                }
                #[inline]
                fn lt(&self, other: &[T, ..$N]) -> bool {
                    PartialOrd::lt(&self[], &other[])
                }
                #[inline]
                fn le(&self, other: &[T, ..$N]) -> bool {
                    PartialOrd::le(&self[], &other[])
                }
                #[inline]
                fn ge(&self, other: &[T, ..$N]) -> bool {
                    PartialOrd::ge(&self[], &other[])
                }
                #[inline]
                fn gt(&self, other: &[T, ..$N]) -> bool {
                    PartialOrd::gt(&self[], &other[])
                }
            }

            #[unstable = "waiting for Ord to stabilize"]
            impl<T:Ord> Ord for [T, ..$N] {
                #[inline]
                fn cmp(&self, other: &[T, ..$N]) -> Ordering {
                    Ord::cmp(&self[], &other[])
                }
            }
        )+
    }
}

array_impls! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

