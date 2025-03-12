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
    fn canonicalize(&mut self) {}
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
    fn union(&self, other: &ClassRange) -> Option<ClassRange> {}
    fn is_contiguous(&self, other: &ClassRange) -> bool {
        let (s1, e1) = (u32::from(self.start), u32::from(self.end));
        let (s2, e2) = (u32::from(other.start), u32::from(other.end));
        core::cmp::max(s1, s2) <= core::cmp::min(e1, e2).saturating_add(1)
    }
    fn is_intersection_empty(&self, other: &ClassRange) -> bool {}
}
