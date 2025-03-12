use alloc::{vec, vec::Vec};
use crate::{
    int::{NonMaxUsize, U32},
    nfa::{State, StateID, NFA},
    pool::CachePoolGuard, utf8,
};
pub(crate) trait U32 {
    fn as_usize(self) -> usize;
}
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
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
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct NonMaxUsize(NonZeroUsize);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Look {
    /// Match the beginning of text. Specifically, this matches at the starting
    /// position of the input.
    Start = 1 << 0,
    /// Match the end of text. Specifically, this matches at the ending
    /// position of the input.
    End = 1 << 1,
    /// Match the beginning of a line or the beginning of text. Specifically,
    /// this matches at the starting position of the input, or at the position
    /// immediately following a `\n` character.
    StartLF = 1 << 2,
    /// Match the end of a line or the end of text. Specifically, this matches
    /// at the end position of the input, or at the position immediately
    /// preceding a `\n` character.
    EndLF = 1 << 3,
    /// Match the beginning of a line or the beginning of text. Specifically,
    /// this matches at the starting position of the input, or at the position
    /// immediately following either a `\r` or `\n` character, but never after
    /// a `\r` when a `\n` follows.
    StartCRLF = 1 << 4,
    /// Match the end of a line or the end of text. Specifically, this matches
    /// at the end position of the input, or at the position immediately
    /// preceding a `\r` or `\n` character, but never before a `\n` when a `\r`
    /// precedes it.
    EndCRLF = 1 << 5,
    /// Match an ASCII-only word boundary. That is, this matches a position
    /// where the left adjacent character and right adjacent character
    /// correspond to a word and non-word or a non-word and word character.
    Word = 1 << 6,
    /// Match an ASCII-only negation of a word boundary.
    WordNegate = 1 << 7,
    /// Match the start of an ASCII-only word boundary. That is, this matches a
    /// position at either the beginning of the haystack or where the previous
    /// character is not a word character and the following character is a word
    /// character.
    WordStart = 1 << 8,
    /// Match the end of an ASCII-only word boundary. That is, this matches
    /// a position at either the end of the haystack or where the previous
    /// character is a word character and the following character is not a word
    /// character.
    WordEnd = 1 << 9,
    /// Match the start half of an ASCII-only word boundary. That is, this
    /// matches a position at either the beginning of the haystack or where the
    /// previous character is not a word character.
    WordStartHalf = 1 << 10,
    /// Match the end half of an ASCII-only word boundary. That is, this
    /// matches a position at either the end of the haystack or where the
    /// following character is not a word character.
    WordEndHalf = 1 << 11,
}
#[derive(Clone, Eq, PartialEq)]
pub(crate) enum State {
    Char { target: StateID, ch: char },
    Ranges { target: StateID, ranges: Vec<(char, char)> },
    Splits { targets: Vec<StateID>, reverse: bool },
    Goto { target: StateID, look: Option<hir::Look> },
    Capture { target: StateID, slot: u32 },
    Fail,
    Match,
}
#[derive(Clone, Debug)]
enum FollowEpsilon {
    /// Explore the epsilon transitions from a state ID.
    Explore(StateID),
    /// Reset the given `slot` to the given `offset` (which might be `None`).
    RestoreCapture { slot: u32, offset: Option<NonMaxUsize> },
}
impl PikeVM {
    pub(crate) fn new(nfa: NFA) -> PikeVM {}
    pub(crate) fn nfa(&self) -> &NFA {}
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
    ) -> bool {}
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
    ) -> bool {}
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
    ) {}
    fn epsilon_closure_explore(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        mut sid: StateID,
    ) {
        loop {
            if !next.set.insert(sid) {
                return;
            }
            match *self.nfa.state(sid) {
                State::Fail
                | State::Match { .. }
                | State::Char { .. }
                | State::Ranges { .. } => {
                    next.slot_table.for_state(sid).copy_from_slice(curr_slots);
                    return;
                }
                State::Goto { target, look: None } => {
                    sid = target;
                }
                State::Goto { target, look: Some(look) } => {
                    if !look.is_match(haystack, at) {
                        return;
                    }
                    sid = target;
                }
                State::Splits { ref targets, reverse: false } => {
                    sid = match targets.get(0) {
                        None => return,
                        Some(&sid) => sid,
                    };
                    stack
                        .extend(
                            targets[1..]
                                .iter()
                                .copied()
                                .rev()
                                .map(FollowEpsilon::Explore),
                        );
                }
                State::Splits { ref targets, reverse: true } => {
                    sid = match targets.last() {
                        None => return,
                        Some(&sid) => sid,
                    };
                    stack
                        .extend(
                            targets[..targets.len() - 1]
                                .iter()
                                .copied()
                                .map(FollowEpsilon::Explore),
                        );
                }
                State::Capture { target, slot } => {
                    if slot.as_usize() < curr_slots.len() {
                        stack
                            .push(FollowEpsilon::RestoreCapture {
                                slot,
                                offset: curr_slots[slot.as_usize()],
                            });
                        curr_slots[slot.as_usize()] = Some(
                            NonMaxUsize::new(at).unwrap(),
                        );
                    }
                    sid = target;
                }
            }
        }
    }
}
impl Look {
    pub(crate) fn is_match(&self, haystack: &[u8], at: usize) -> bool {
        use self::Look::*;
        match *self {
            Start => at == 0,
            End => at == haystack.len(),
            StartLF => at == 0 || haystack[at - 1] == b'\n',
            EndLF => at == haystack.len() || haystack[at] == b'\n',
            StartCRLF => {
                at == 0 || haystack[at - 1] == b'\n'
                    || (haystack[at - 1] == b'\r'
                        && (at >= haystack.len() || haystack[at] != b'\n'))
            }
            EndCRLF => {
                at == haystack.len() || haystack[at] == b'\r'
                    || (haystack[at] == b'\n' && (at == 0 || haystack[at - 1] != b'\r'))
            }
            Word => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                word_before != word_after
            }
            WordNegate => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                word_before == word_after
            }
            WordStart => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                !word_before && word_after
            }
            WordEnd => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                word_before && !word_after
            }
            WordStartHalf => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                !word_before
            }
            WordEndHalf => {
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                !word_after
            }
        }
    }
}
impl SlotTable {
    fn new() -> SlotTable {}
    fn reset(&mut self, re: &PikeVM) {}
    fn setup_search(&mut self, captures_slot_len: usize) {}
    fn for_state(&mut self, sid: StateID) -> &mut [Option<NonMaxUsize>] {
        let i = sid.as_usize() * self.slots_per_state;
        &mut self.table[i..i + self.slots_for_captures]
    }
    fn all_absent(&mut self) -> &mut [Option<NonMaxUsize>] {}
}
impl NonMaxUsize {
    pub(crate) fn new(value: usize) -> Option<NonMaxUsize> {
        NonZeroUsize::new(value.wrapping_add(1)).map(NonMaxUsize)
    }
    pub(crate) fn get(self) -> usize {}
}
impl NFA {
    pub(crate) fn new(config: Config, pattern: String, hir: &Hir) -> Result<NFA, Error> {}
    pub(crate) fn pattern(&self) -> &str {}
    pub(crate) fn state(&self, id: StateID) -> &State {
        &self.states[id.as_usize()]
    }
    pub(crate) fn len(&self) -> usize {}
    pub(crate) fn start(&self) -> StateID {}
    pub(crate) fn to_index(&self, name: &str) -> Option<usize> {}
    pub(crate) fn capture_names(&self) -> CaptureNames<'_> {}
    pub(crate) fn group_len(&self) -> usize {}
    pub(crate) fn is_start_anchored(&self) -> bool {}
    pub(crate) fn static_explicit_captures_len(&self) -> Option<usize> {}
    fn memory_usage(&self) -> usize {}
}
impl SparseSet {
    fn new(capacity: usize) -> SparseSet {}
    fn resize(&mut self, new_capacity: usize) {}
    fn capacity(&self) -> usize {}
    fn len(&self) -> usize {}
    fn is_empty(&self) -> bool {}
    fn insert(&mut self, id: StateID) -> bool {
        if self.contains(id) {
            return false;
        }
        let index = self.len();
        assert!(
            index < self.capacity(), "{:?} exceeds capacity of {:?} when inserting {:?}",
            index, self.capacity(), id,
        );
        self.dense[index] = id;
        self.sparse[id.as_usize()] = u32::try_from(index).unwrap();
        self.len += 1;
        true
    }
    fn contains(&self, id: StateID) -> bool {}
    fn clear(&mut self) {}
    fn iter(&self) -> SparseSetIter<'_> {}
}
