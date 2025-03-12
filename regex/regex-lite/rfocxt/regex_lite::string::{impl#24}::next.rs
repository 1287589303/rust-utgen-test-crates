use alloc::{
    borrow::Cow, boxed::Box, string::String, string::ToString, sync::Arc, vec, vec::Vec,
};
use crate::{
    error::Error, hir::{self, Hir},
    int::NonMaxUsize, interpolate, nfa::{self, NFA},
    pikevm::{self, Cache, PikeVM},
    pool::CachePool,
};
#[derive(Debug)]
pub struct SplitN<'r, 'h> {
    splits: Split<'r, 'h>,
    limit: usize,
}
#[derive(Debug)]
pub struct Split<'r, 'h> {
    haystack: &'h str,
    finder: Matches<'r, 'h>,
    last: usize,
}
impl<'r, 'h> Iterator for SplitN<'r, 'h> {
    type Item = &'h str;
    #[inline]
    fn next(&mut self) -> Option<&'h str> {
        if self.limit == 0 {
            return None;
        }
        self.limit -= 1;
        if self.limit > 0 {
            return self.splits.next();
        }
        let len = self.splits.haystack.len();
        if self.splits.last > len {
            None
        } else {
            Some(&self.splits.haystack[self.splits.last..len])
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {}
}
