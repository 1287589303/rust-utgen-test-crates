use core::{cell::RefCell, fmt, mem, ops::RangeInclusive};
use alloc::{format, string::String, vec, vec::Vec};
use regex_syntax::utf8::Utf8Range;
use crate::util::primitives::StateID;
const FINAL: StateID = StateID::ZERO;
const ROOT: StateID = StateID::new_unchecked(1);
#[derive(Clone)]
struct State {
    /// A sorted sequence of non-overlapping transitions to other states. Each
    /// transition corresponds to a single range of bytes.
    transitions: Vec<Transition>,
}
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
}
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
#[derive(Clone)]
struct Transition {
    /// The byte range.
    range: Utf8Range,
    /// The next state to transition to.
    next_id: StateID,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
impl State {
    fn find(&self, range: Utf8Range) -> usize {
        /// Returns the position `i` at which `pred(xs[i])` first returns true
        /// such that for all `j >= i`, `pred(xs[j]) == true`. If `pred` never
        /// returns true, then `xs.len()` is returned.
        ///
        /// We roll our own binary search because it doesn't seem like the
        /// standard library's binary search can be used here. Namely, if
        /// there is an overlapping range, then we want to find the first such
        /// occurrence, but there may be many. Or at least, it's not quite
        /// clear to me how to do it.
        fn binary_search<T, F>(xs: &[T], mut pred: F) -> usize
        where
            F: FnMut(&T) -> bool,
        {
            let (mut left, mut right) = (0, xs.len());
            while left < right {
                let mid = (left + right) / 2;
                if pred(&xs[mid]) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        }
        binary_search(&self.transitions, |t| range.start <= t.range.end)
    }
    fn clear(&mut self) {}
}
