use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ClassRange {
    pub(crate) start: char,
    pub(crate) end: char,
}
impl ClassRange {
    fn ascii_case_fold(&self) -> Option<ClassRange> {
        if !(ClassRange { start: 'a', end: 'z' }).is_intersection_empty(self) {
            let start = core::cmp::max(self.start, 'a');
            let end = core::cmp::min(self.end, 'z');
            return Some(ClassRange {
                start: char::try_from(u32::from(start) - 32).unwrap(),
                end: char::try_from(u32::from(end) - 32).unwrap(),
            });
        }
        if !(ClassRange { start: 'A', end: 'Z' }).is_intersection_empty(self) {
            let start = core::cmp::max(self.start, 'A');
            let end = core::cmp::min(self.end, 'Z');
            return Some(ClassRange {
                start: char::try_from(u32::from(start) + 32).unwrap(),
                end: char::try_from(u32::from(end) + 32).unwrap(),
            });
        }
        None
    }
    fn union(&self, other: &ClassRange) -> Option<ClassRange> {}
    fn is_contiguous(&self, other: &ClassRange) -> bool {}
    fn is_intersection_empty(&self, other: &ClassRange) -> bool {
        let (s1, e1) = (self.start, self.end);
        let (s2, e2) = (other.start, other.end);
        core::cmp::max(s1, s2) > core::cmp::min(e1, e2)
    }
}
