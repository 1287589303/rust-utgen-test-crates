use alloc::{vec, vec::Vec};
use crate::{
    int::{NonMaxUsize, U32},
    nfa::{State, StateID, NFA},
    pool::CachePoolGuard, utf8,
};
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
}
#[derive(Clone)]
struct SparseSet {
    /// The number of elements currently in this set.
    len: usize,
    /// Dense contains the ids in the order in which they were inserted.
    dense: Vec<StateID>,
    /// Sparse maps ids to their location in dense.
    ///
    /// A state ID is in the set if and only if
    /// sparse[id] < len && id == dense[sparse[id]].
    ///
    /// Note that these are indices into 'dense'. It's a little weird to use
    /// StateID here, but we know our length can never exceed the bounds of
    /// StateID (enforced by 'resize') and StateID will be at most 4 bytes
    /// where as a usize is likely double that in most cases.
    sparse: Vec<StateID>,
}
#[derive(Clone, Debug)]
struct SlotTable {
    /// The actual table of offsets.
    table: Vec<Option<NonMaxUsize>>,
    /// The number of slots per state, i.e., the table's stride or the length
    /// of each row.
    slots_per_state: usize,
    /// The number of slots in the caller-provided `Captures` value for the
    /// current search. Setting this to `slots_per_state` is always correct,
    /// but may be wasteful.
    slots_for_captures: usize,
}
#[derive(Clone)]
pub(crate) struct NFA {
    /// The pattern string this NFA was generated from.
    ///
    /// We put it here for lack of a better place to put it. ¯\_(ツ)_/¯
    pattern: String,
    /// The states that make up this NFA.
    states: Vec<State>,
    /// The ID of the start state.
    start: StateID,
    /// Whether this NFA can only match at the beginning of a haystack.
    is_start_anchored: bool,
    /// Whether this NFA can match the empty string.
    is_match_empty: bool,
    /// If every match has the same number of matching capture groups, then
    /// this corresponds to the number of groups.
    static_explicit_captures_len: Option<usize>,
    /// A map from capture group name to its corresponding index.
    cap_name_to_index: CaptureNameMap,
    /// A map from capture group index to the corresponding name, if one
    /// exists.
    cap_index_to_name: Vec<Option<Arc<str>>>,
    /// Heap memory used indirectly by NFA states and other things (like the
    /// various capturing group representations above). Since each state
    /// might use a different amount of heap, we need to keep track of this
    /// incrementally.
    memory_extra: usize,
}
#[derive(Clone, Debug)]
pub(crate) struct Cache {
    /// Stack used while computing epsilon closure. This effectively lets us
    /// move what is more naturally expressed through recursion to a stack
    /// on the heap.
    stack: Vec<FollowEpsilon>,
    /// The current active states being explored for the current byte in the
    /// haystack.
    curr: ActiveStates,
    /// The next set of states we're building that will be explored for the
    /// next byte in the haystack.
    next: ActiveStates,
}
#[derive(Clone, Debug)]
struct ActiveStates {
    /// The set of active NFA states. This set preserves insertion order, which
    /// is critical for simulating the match semantics of backtracking regex
    /// engines.
    set: SparseSet,
    /// The slots for every NFA state, where each slot stores a (possibly
    /// absent) offset. Every capturing group has two slots. One for a start
    /// offset and one for an end offset.
    slot_table: SlotTable,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct NonMaxUsize(NonZeroUsize);
#[derive(Clone, Debug)]
enum FollowEpsilon {
    /// Explore the epsilon transitions from a state ID.
    Explore(StateID),
    /// Reset the given `slot` to the given `offset` (which might be `None`).
    RestoreCapture { slot: u32, offset: Option<NonMaxUsize> },
}
impl PikeVM {
    pub(crate) fn new(nfa: NFA) -> PikeVM {}
    pub(crate) fn nfa(&self) -> &NFA {
        &self.nfa
    }
    pub(crate) fn find_iter<'r, 'h>(
        &'r self,
        cache: CachePoolGuard<'r>,
        haystack: &'h [u8],
    ) -> FindMatches<'r, 'h> {}
    pub(crate) fn captures_iter<'r, 'h>(
        &'r self,
        cache: CachePoolGuard<'r>,
        haystack: &'h [u8],
    ) -> CapturesMatches<'r, 'h> {}
    pub(crate) fn search(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
        earliest: bool,
        slots: &mut [Option<NonMaxUsize>],
    ) -> bool {
        cache.setup_search(slots.len());
        if start > end {
            return false;
        }
        assert!(
            haystack.len() < core::usize::MAX,
            "byte slice lengths must be less than usize MAX",
        );
        let Cache { ref mut stack, ref mut curr, ref mut next } = cache;
        let start_id = self.nfa().start();
        let anchored = self.nfa().is_start_anchored();
        let mut matched = false;
        let mut at = start;
        while at <= end {
            if curr.set.is_empty() {
                if matched {
                    break;
                }
                if anchored && at > start {
                    break;
                }
            }
            if !matched {
                let slots = next.slot_table.all_absent();
                self.epsilon_closure(stack, slots, curr, haystack, at, start_id);
            }
            let (ch, len) = utf8::decode_lossy(&haystack[at..]);
            if self.nexts(stack, curr, next, haystack, at, ch, len, slots) {
                matched = true;
            }
            if (earliest && matched) || len == 0 {
                break;
            }
            core::mem::swap(curr, next);
            next.set.clear();
            at += len;
        }
        matched
    }
    fn nexts(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr: &mut ActiveStates,
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        at_ch: char,
        at_len: usize,
        slots: &mut [Option<NonMaxUsize>],
    ) -> bool {
        let ActiveStates { ref set, ref mut slot_table } = *curr;
        for sid in set.iter() {
            if self.next(stack, slot_table, next, haystack, at, at_ch, at_len, sid) {
                slots.copy_from_slice(slot_table.for_state(sid));
                return true;
            }
        }
        false
    }
    fn next(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slot_table: &mut SlotTable,
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        at_ch: char,
        at_len: usize,
        sid: StateID,
    ) -> bool {}
    fn epsilon_closure(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        sid: StateID,
    ) {
        stack.push(FollowEpsilon::Explore(sid));
        while let Some(frame) = stack.pop() {
            match frame {
                FollowEpsilon::RestoreCapture { slot, offset } => {
                    curr_slots[slot.as_usize()] = offset;
                }
                FollowEpsilon::Explore(sid) => {
                    self.epsilon_closure_explore(
                        stack,
                        curr_slots,
                        next,
                        haystack,
                        at,
                        sid,
                    );
                }
            }
        }
    }
    fn epsilon_closure_explore(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        mut sid: StateID,
    ) {}
}
impl SparseSet {
    fn new(capacity: usize) -> SparseSet {}
    fn resize(&mut self, new_capacity: usize) {}
    fn capacity(&self) -> usize {}
    fn len(&self) -> usize {}
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn insert(&mut self, id: StateID) -> bool {}
    fn contains(&self, id: StateID) -> bool {}
    fn clear(&mut self) {
        self.len = 0;
    }
    fn iter(&self) -> SparseSetIter<'_> {}
}
impl SlotTable {
    fn new() -> SlotTable {}
    fn reset(&mut self, re: &PikeVM) {}
    fn setup_search(&mut self, captures_slot_len: usize) {}
    fn for_state(&mut self, sid: StateID) -> &mut [Option<NonMaxUsize>] {}
    fn all_absent(&mut self) -> &mut [Option<NonMaxUsize>] {
        let i = self.table.len() - self.slots_per_state;
        &mut self.table[i..i + self.slots_for_captures]
    }
}
impl NFA {
    pub(crate) fn new(config: Config, pattern: String, hir: &Hir) -> Result<NFA, Error> {}
    pub(crate) fn pattern(&self) -> &str {}
    pub(crate) fn state(&self, id: StateID) -> &State {}
    pub(crate) fn len(&self) -> usize {}
    pub(crate) fn start(&self) -> StateID {
        self.start
    }
    pub(crate) fn to_index(&self, name: &str) -> Option<usize> {}
    pub(crate) fn capture_names(&self) -> CaptureNames<'_> {}
    pub(crate) fn group_len(&self) -> usize {}
    pub(crate) fn is_start_anchored(&self) -> bool {
        self.is_start_anchored
    }
    pub(crate) fn static_explicit_captures_len(&self) -> Option<usize> {}
    fn memory_usage(&self) -> usize {}
}
impl Cache {
    pub(crate) fn new(re: &PikeVM) -> Cache {}
    fn setup_search(&mut self, captures_slot_len: usize) {
        self.stack.clear();
        self.curr.setup_search(captures_slot_len);
        self.next.setup_search(captures_slot_len);
    }
}
pub(crate) fn decode_lossy<B: AsRef<[u8]>>(slice: B) -> (char, usize) {
    match decode(slice) {
        (Some(ch), size) => (ch, size),
        (None, size) => ('\u{FFFD}', size),
    }
}
