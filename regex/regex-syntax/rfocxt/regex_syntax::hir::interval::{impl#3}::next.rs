use core::{char, cmp, fmt::Debug, slice};
use alloc::vec::Vec;
use crate::unicode;
#[derive(Debug)]
pub struct IntervalSetIter<'a, I>(slice::Iter<'a, I>);
impl<'a, I> Iterator for IntervalSetIter<'a, I> {
    type Item = &'a I;
    fn next(&mut self) -> Option<&'a I> {
        self.0.next()
    }
}
