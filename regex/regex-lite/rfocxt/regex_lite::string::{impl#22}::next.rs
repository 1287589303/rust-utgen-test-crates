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
pub struct Split<'r, 'h> {
    haystack: &'h str,
    finder: Matches<'r, 'h>,
    last: usize,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
#[derive(Debug)]
pub struct Matches<'r, 'h> {
    haystack: &'h str,
    it: pikevm::FindMatches<'r, 'h>,
}
impl<'r, 'h> Iterator for Split<'r, 'h> {
    type Item = &'h str;
    #[inline]
    fn next(&mut self) -> Option<&'h str> {
        match self.finder.next() {
            None => {
                let len = self.haystack.len();
                if self.last > len {
                    None
                } else {
                    let range = self.last..len;
                    self.last = len + 1;
                    Some(&self.haystack[range])
                }
            }
            Some(m) => {
                let range = self.last..m.start();
                self.last = m.end();
                Some(&self.haystack[range])
            }
        }
    }
}
impl<'h> Match<'h> {
    #[inline]
    fn new(haystack: &'h str, start: usize, end: usize) -> Match<'h> {}
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {}
    #[inline]
    pub fn as_str(&self) -> &'h str {}
}
