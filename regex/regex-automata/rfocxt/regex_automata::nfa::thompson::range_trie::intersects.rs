use core::{cell::RefCell, fmt, mem, ops::RangeInclusive};
use alloc::{format, string::String, vec, vec::Vec};
use regex_syntax::utf8::Utf8Range;
use crate::util::primitives::StateID;
const FINAL: StateID = StateID::ZERO;
const ROOT: StateID = StateID::new_unchecked(1);
fn intersects(r1: Utf8Range, r2: Utf8Range) -> bool {
    !(r1.end < r2.start || r2.end < r1.start)
}
