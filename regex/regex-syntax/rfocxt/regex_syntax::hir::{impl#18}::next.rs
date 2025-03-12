use core::{char, cmp};
use alloc::{
    boxed::Box, format, string::{String, ToString},
    vec, vec::Vec,
};
use crate::{
    ast::Span, hir::interval::{Interval, IntervalSet, IntervalSetIter},
    unicode,
};
pub use crate::{
    hir::visitor::{visit, Visitor},
    unicode::CaseFoldError,
};
#[derive(Debug)]
pub struct ClassBytesIter<'a>(IntervalSetIter<'a, ClassBytesRange>);
#[derive(Debug)]
pub struct IntervalSetIter<'a, I>(slice::Iter<'a, I>);
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassBytesRange {
    start: u8,
    end: u8,
}
impl<'a> Iterator for ClassBytesIter<'a> {
    type Item = &'a ClassBytesRange;
    fn next(&mut self) -> Option<&'a ClassBytesRange> {
        self.0.next()
    }
}
