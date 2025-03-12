use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::{self, BuildError, State, NFA},
    util::{
        captures::Captures, empty, iter, prefilter::Prefilter,
        primitives::{NonMaxUsize, PatternID, SmallIndex, StateID},
        search::{Anchored, HalfMatch, Input, Match, MatchError, Span},
    },
};
#[derive(Clone, Debug)]
struct Visited {
    /// The actual underlying bitset. Each element in the bitset corresponds
    /// to a particular (StateID, offset) pair. States correspond to the rows
    /// and the offsets correspond to the columns.
    ///
    /// If our underlying NFA has N states and the haystack we're searching
    /// has M bytes, then we have N*(M+1) entries in our bitset table. The
    /// M+1 occurs because our matches are delayed by one byte (to support
    /// look-around), and so we need to handle the end position itself rather
    /// than stopping just before the end. (If there is no end position, then
    /// it's treated as "end-of-input," which is matched by things like '$'.)
    ///
    /// Given BITS=N*(M+1), we wind up with div_ceil(BITS, sizeof(usize))
    /// blocks.
    ///
    /// We use 'usize' to represent our blocks because it makes some of the
    /// arithmetic in 'insert' a bit nicer. For example, if we used 'u32' for
    /// our block, we'd either need to cast u32s to usizes or usizes to u32s.
    bitset: Vec<usize>,
    /// The stride represents one plus length of the haystack we're searching
    /// (as described above). The stride must be initialized for each search.
    stride: usize,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl Visited {
    const BLOCK_SIZE: usize = 8 * core::mem::size_of::<usize>();
    fn new(re: &BoundedBacktracker) -> Visited {}
    fn insert(&mut self, sid: StateID, at: usize) -> bool {
        let table_index = sid.as_usize() * self.stride + at;
        let block_index = table_index / Visited::BLOCK_SIZE;
        let bit = table_index % Visited::BLOCK_SIZE;
        let block_with_bit = 1 << bit;
        if self.bitset[block_index] & block_with_bit != 0 {
            return false;
        }
        self.bitset[block_index] |= block_with_bit;
        true
    }
    fn reset(&mut self, _: &BoundedBacktracker) {}
    fn setup_search(
        &mut self,
        re: &BoundedBacktracker,
        input: &Input<'_>,
    ) -> Result<(), MatchError> {}
    fn memory_usage(&self) -> usize {}
}
