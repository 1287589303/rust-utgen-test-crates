use core::{cell::RefCell, fmt, mem, ops::RangeInclusive};
use alloc::{format, string::String, vec, vec::Vec};
use regex_syntax::utf8::Utf8Range;
use crate::util::primitives::StateID;
const FINAL: StateID = StateID::ZERO;
const ROOT: StateID = StateID::new_unchecked(1);
#[derive(Clone, Debug, Eq, PartialEq)]
struct Split {
    partitions: [SplitRange; 3],
    len: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SplitRange {
    Old(Utf8Range),
    New(Utf8Range),
    Both(Utf8Range),
}
impl Split {
    fn new(o: Utf8Range, n: Utf8Range) -> Option<Split> {}
    fn parts1(r1: SplitRange) -> Split {}
    fn parts2(r1: SplitRange, r2: SplitRange) -> Split {}
    fn parts3(r1: SplitRange, r2: SplitRange, r3: SplitRange) -> Split {}
    fn as_slice(&self) -> &[SplitRange] {
        &self.partitions[..self.len]
    }
}
