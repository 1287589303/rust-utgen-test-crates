use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ClassRange {
    pub(crate) start: char,
    pub(crate) end: char,
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
    fn is_contiguous(&self, other: &ClassRange) -> bool {
        let (s1, e1) = (u32::from(self.start), u32::from(self.end));
        let (s2, e2) = (u32::from(other.start), u32::from(other.end));
        core::cmp::max(s1, s2) <= core::cmp::min(e1, e2).saturating_add(1)
    }
    fn is_intersection_empty(&self, other: &ClassRange) -> bool {}
}
