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
    fn ascii_case_fold(&mut self) {
        let len = self.ranges.len();
        for i in 0..len {
            if let Some(folded) = self.ranges[i].ascii_case_fold() {
                self.ranges.push(folded);
            }
        }
        self.canonicalize();
    }
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
    fn is_canonical(&self) -> bool {}
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
    fn is_intersection_empty(&self, other: &ClassRange) -> bool {}
}
