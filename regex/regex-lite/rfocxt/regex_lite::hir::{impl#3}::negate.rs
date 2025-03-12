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
    fn negate(&mut self) {
        const MIN: char = '\x00';
        const MAX: char = char::MAX;
        if self.ranges.is_empty() {
            self.ranges.push(ClassRange { start: MIN, end: MAX });
            return;
        }
        let drain_end = self.ranges.len();
        if self.ranges[0].start > MIN {
            self.ranges
                .push(ClassRange {
                    start: MIN,
                    end: prev_char(self.ranges[0].start).unwrap(),
                });
        }
        for i in 1..drain_end {
            self.ranges
                .push(ClassRange {
                    start: next_char(self.ranges[i - 1].end).unwrap(),
                    end: prev_char(self.ranges[i].start).unwrap(),
                });
        }
        if self.ranges[drain_end - 1].end < MAX {
            self.ranges
                .push(ClassRange {
                    start: next_char(self.ranges[drain_end - 1].end).unwrap(),
                    end: MAX,
                });
        }
        self.ranges.drain(..drain_end);
    }
    fn canonicalize(&mut self) {}
    fn is_canonical(&self) -> bool {}
}
fn prev_char(ch: char) -> Option<char> {
    if ch == '\u{E000}' {
        return Some('\u{D7FF}');
    }
    Some(char::from_u32(u32::from(ch).checked_sub(1)?).unwrap())
}
fn next_char(ch: char) -> Option<char> {
    if ch == '\u{D7FF}' {
        return Some('\u{E000}');
    }
    char::from_u32(u32::from(ch).checked_add(1).unwrap())
}
