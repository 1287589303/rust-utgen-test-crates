use core::{char, fmt, iter::FusedIterator, slice};
use alloc::{vec, vec::Vec};
const MAX_UTF8_BYTES: usize = 4;
#[derive(Debug)]
pub struct Utf8Sequences {
    range_stack: Vec<ScalarRange>,
}
struct ScalarRange {
    start: u32,
    end: u32,
}
impl Utf8Sequences {
    pub fn new(start: char, end: char) -> Self {
        let range = ScalarRange {
            start: u32::from(start),
            end: u32::from(end),
        };
        Utf8Sequences {
            range_stack: vec![range],
        }
    }
    pub fn reset(&mut self, start: char, end: char) {
        self.range_stack.clear();
        self.push(u32::from(start), u32::from(end));
    }
    fn push(&mut self, start: u32, end: u32) {
        self.range_stack.push(ScalarRange { start, end });
    }
}
