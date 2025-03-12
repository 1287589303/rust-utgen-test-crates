use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ClassRange {
    pub(crate) start: char,
    pub(crate) end: char,
}
impl ClassRange {
    fn ascii_case_fold(&self) -> Option<ClassRange> {}
    fn union(&self, other: &ClassRange) -> Option<ClassRange> {}
    fn is_contiguous(&self, other: &ClassRange) -> bool {}
    fn is_intersection_empty(&self, other: &ClassRange) -> bool {
        let (s1, e1) = (self.start, self.end);
        let (s2, e2) = (other.start, other.end);
        core::cmp::max(s1, s2) > core::cmp::min(e1, e2)
    }
}
