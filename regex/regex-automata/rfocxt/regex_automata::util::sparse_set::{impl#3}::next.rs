use alloc::{vec, vec::Vec};
use crate::util::primitives::StateID;
#[derive(Debug)]
pub(crate) struct SparseSetIter<'a>(core::slice::Iter<'a, StateID>);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl<'a> Iterator for SparseSetIter<'a> {
    type Item = StateID;
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn next(&mut self) -> Option<StateID> {
        self.0.next().map(|&id| id)
    }
}
