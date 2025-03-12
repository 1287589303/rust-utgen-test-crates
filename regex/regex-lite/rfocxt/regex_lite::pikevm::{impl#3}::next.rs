use alloc::{vec, vec::Vec};
use crate::{
    int::{NonMaxUsize, U32},
    nfa::{State, StateID, NFA},
    pool::CachePoolGuard, utf8,
};
#[derive(Debug)]
pub(crate) struct CapturesMatches<'r, 'h> {
    it: FindMatches<'r, 'h>,
}
#[derive(Debug)]
pub(crate) struct FindMatches<'r, 'h> {
    pikevm: &'r PikeVM,
    cache: CachePoolGuard<'r>,
    haystack: &'h [u8],
    at: usize,
    slots: Vec<Option<NonMaxUsize>>,
    last_match_end: Option<usize>,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct NonMaxUsize(NonZeroUsize);
impl<'r, 'h> Iterator for CapturesMatches<'r, 'h> {
    type Item = Vec<Option<NonMaxUsize>>;
    fn next(&mut self) -> Option<Vec<Option<NonMaxUsize>>> {
        self.it.next()?;
        Some(self.it.slots.clone())
    }
}
