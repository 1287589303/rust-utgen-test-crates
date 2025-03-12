use alloc::{vec, vec::Vec};
use crate::{
    int::{NonMaxUsize, U32},
    nfa::{State, StateID, NFA},
    pool::CachePoolGuard, utf8,
};
#[derive(Debug)]
struct SparseSetIter<'a>(core::slice::Iter<'a, StateID>);
impl<'a> Iterator for SparseSetIter<'a> {
    type Item = StateID;
    fn next(&mut self) -> Option<StateID> {
        self.0.next().map(|&id| id)
    }
}
