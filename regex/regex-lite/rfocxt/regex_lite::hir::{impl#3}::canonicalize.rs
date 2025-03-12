use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Class {
    pub(crate) ranges: Vec<ClassRange>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ClassRange {
    pub(crate) start: char,
    pub(crate) end: char,
}
impl Class {
    fn new<I: IntoIterator<Item = ClassRange>>(ranges: I) -> Class {}
    fn ascii_case_fold(&mut self) {}
    fn negate(&mut self) {}
    fn canonicalize(&mut self) {
        if self.is_canonical() {
            return;
        }
        self.ranges.sort();
        assert!(! self.ranges.is_empty());
        let drain_end = self.ranges.len();
        for oldi in 0..drain_end {
            if self.ranges.len() > drain_end {
                let (last, rest) = self.ranges.split_last_mut().unwrap();
                if let Some(union) = last.union(&rest[oldi]) {
                    *last = union;
                    continue;
                }
            }
            self.ranges.push(self.ranges[oldi]);
        }
        self.ranges.drain(..drain_end);
    }
    fn is_canonical(&self) -> bool {
        for pair in self.ranges.windows(2) {
            if pair[0] >= pair[1] {
                return false;
            }
            if pair[0].is_contiguous(&pair[1]) {
                return false;
            }
        }
        true
    }
}
impl ClassRange {
    fn ascii_case_fold(&self) -> Option<ClassRange> {}
    fn union(&self, other: &ClassRange) -> Option<ClassRange> {
        if !self.is_contiguous(other) {
            return None;
        }
        let start = core::cmp::min(self.start, other.start);
        let end = core::cmp::max(self.end, other.end);
        Some(ClassRange { start, end })
    }
    fn is_contiguous(&self, other: &ClassRange) -> bool {}
    fn is_intersection_empty(&self, other: &ClassRange) -> bool {}
}
