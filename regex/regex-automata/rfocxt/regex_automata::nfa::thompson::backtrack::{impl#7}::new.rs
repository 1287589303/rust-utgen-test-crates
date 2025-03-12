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
pub struct Cache {
    /// Stack used on the heap for doing backtracking instead of the
    /// traditional recursive approach. We don't want recursion because then
    /// we're likely to hit a stack overflow for bigger regexes.
    stack: Vec<Frame>,
    /// The set of (StateID, HaystackOffset) pairs that have been visited
    /// by the backtracker within a single search. If such a pair has been
    /// visited, then we avoid doing the work for that pair again. This is
    /// what "bounds" the backtracking and prevents it from having worst case
    /// exponential time.
    visited: Visited,
}
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
#[derive(Clone, Debug)]
pub struct BoundedBacktracker {
    config: Config,
    nfa: NFA,
}
#[derive(Debug)]
struct Frame<'a> {
    /// The remaining chunks to visit for a trie state.
    chunks: StateChunksIter<'a>,
    /// The transitions of the current chunk that we're iterating over. Since
    /// every trie state has at least one chunk, every frame is initialized
    /// with the first chunk's transitions ready to be consumed.
    transitions: core::slice::Iter<'a, Transition>,
    /// The NFA state IDs pointing to the start of each chunk compiled by
    /// this trie state. This ultimately gets converted to an NFA union once
    /// the entire trie state (and all of its children) have been compiled.
    /// The order of these matters for leftmost-first match semantics, since
    /// earlier matches in the union are preferred over later ones.
    union: Vec<StateID>,
    /// The actual NFA transitions for a single chunk in a trie state. This
    /// gets converted to an NFA sparse state, and its corresponding NFA state
    /// ID should get added to 'union'.
    sparse: Vec<thompson::Transition>,
}
#[derive(Clone, Debug)]
enum Frame {
    /// Look for a match starting at `sid` and the given position in the
    /// haystack.
    Step { sid: StateID, at: usize },
    /// Reset the given `slot` to the given `offset` (which might be `None`).
    /// This effectively gives a "scope" to capturing groups, such that an
    /// offset for a particular group only gets returned if the match goes
    /// through that capturing group. If backtracking ends up going down a
    /// different branch that results in a different offset (or perhaps none at
    /// all), then this "restore capture" frame will cause the offset to get
    /// reset.
    RestoreCapture { slot: SmallIndex, offset: Option<NonMaxUsize> },
}
impl Cache {
    pub fn new(re: &BoundedBacktracker) -> Cache {
        Cache {
            stack: vec![],
            visited: Visited::new(re),
        }
    }
    pub fn reset(&mut self, re: &BoundedBacktracker) {}
    pub fn memory_usage(&self) -> usize {}
    fn setup_search(
        &mut self,
        re: &BoundedBacktracker,
        input: &Input<'_>,
    ) -> Result<(), MatchError> {}
}
impl Visited {
    const BLOCK_SIZE: usize = 8 * core::mem::size_of::<usize>();
    fn new(re: &BoundedBacktracker) -> Visited {
        let mut visited = Visited {
            bitset: vec![],
            stride: 0,
        };
        visited.reset(re);
        visited
    }
    fn insert(&mut self, sid: StateID, at: usize) -> bool {}
    fn reset(&mut self, _: &BoundedBacktracker) {}
    fn setup_search(
        &mut self,
        re: &BoundedBacktracker,
        input: &Input<'_>,
    ) -> Result<(), MatchError> {}
    fn memory_usage(&self) -> usize {}
}
