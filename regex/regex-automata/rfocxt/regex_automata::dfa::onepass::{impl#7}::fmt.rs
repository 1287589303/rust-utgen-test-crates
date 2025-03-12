use alloc::{vec, vec::Vec};
use crate::{
    dfa::{remapper::Remapper, DEAD},
    nfa::thompson::{self, NFA},
    util::{
        alphabet::ByteClasses, captures::Captures, escape::DebugByte,
        int::{Usize, U32, U64, U8},
        look::{Look, LookSet, UnicodeWordBoundaryError},
        primitives::{NonMaxUsize, PatternID, StateID},
        search::{Anchored, Input, Match, MatchError, MatchKind, Span},
        sparse_set::SparseSet,
    },
};
pub(super) trait Remappable: core::fmt::Debug {
    fn state_len(&self) -> usize;
    fn stride2(&self) -> usize;
    fn swap_states(&mut self, id1: StateID, id2: StateID);
    fn remap(&mut self, map: impl Fn(StateID) -> StateID);
}
#[derive(Clone)]
pub struct DFA {
    /// The configuration provided by the caller.
    config: Config,
    /// The NFA used to build this DFA.
    ///
    /// NOTE: We probably don't need to store the NFA here, but we use enough
    /// bits from it that it's convenient to do so. And there really isn't much
    /// cost to doing so either, since an NFA is reference counted internally.
    nfa: NFA,
    /// The transition table. Given a state ID 's' and a byte of haystack 'b',
    /// the next state is `table[sid + classes[byte]]`.
    ///
    /// The stride of this table (i.e., the number of columns) is always
    /// a power of 2, even if the alphabet length is smaller. This makes
    /// converting between state IDs and state indices very cheap.
    ///
    /// Note that the stride always includes room for one extra "transition"
    /// that isn't actually a transition. It is a 'PatternEpsilons' that is
    /// used for match states only. Because of this, the maximum number of
    /// active columns in the transition table is 257, which means the maximum
    /// stride is 512 (the next power of 2 greater than or equal to 257).
    table: Vec<Transition>,
    /// The DFA state IDs of the starting states.
    ///
    /// `starts[0]` is always present and corresponds to the starting state
    /// when searching for matches of any pattern in the DFA.
    ///
    /// `starts[i]` where i>0 corresponds to the starting state for the pattern
    /// ID 'i-1'. These starting states are optional.
    starts: Vec<StateID>,
    /// Every state ID >= this value corresponds to a match state.
    ///
    /// This is what a search uses to detect whether a state is a match state
    /// or not. It requires only a simple comparison instead of bit-unpacking
    /// the PatternEpsilons from every state.
    min_match_id: StateID,
    /// The alphabet of this DFA, split into equivalence classes. Bytes in the
    /// same equivalence class can never discriminate between a match and a
    /// non-match.
    classes: ByteClasses,
    /// The number of elements in each state in the transition table. This may
    /// be less than the stride, since the stride is always a power of 2 and
    /// the alphabet length can be anything up to and including 256.
    alphabet_len: usize,
    /// The number of columns in the transition table, expressed as a power of
    /// 2.
    stride2: usize,
    /// The offset at which the PatternEpsilons for a match state is stored in
    /// the transition table.
    ///
    /// PERF: One wonders whether it would be better to put this in a separate
    /// allocation, since only match states have a non-empty PatternEpsilons
    /// and the number of match states tends be dwarfed by the number of
    /// non-match states. So this would save '8*len(non_match_states)' for each
    /// DFA. The question is whether moving this to a different allocation will
    /// lead to a perf hit during searches. You might think dealing with match
    /// states is rare, but some regexes spend a lot of time in match states
    /// gobbling up input. But... match state handling is already somewhat
    /// expensive, so maybe this wouldn't do much? Either way, it's worth
    /// experimenting.
    pateps_offset: usize,
    /// The first explicit slot index. This refers to the first slot appearing
    /// immediately after the last implicit slot. It is always 'patterns.len()
    /// * 2'.
    ///
    /// We record this because we only store the explicit slots in our DFA
    /// transition table that need to be saved. Implicit slots are handled
    /// automatically as part of the search.
    explicit_slot_start: usize,
}
#[derive(Clone, Copy)]
struct PatternEpsilons(u64);
#[derive(Clone, Debug, Default)]
pub struct Config {
    utf8: Option<bool>,
    reverse: Option<bool>,
    nfa_size_limit: Option<Option<usize>>,
    shrink: Option<bool>,
    which_captures: Option<WhichCaptures>,
    look_matcher: Option<LookMatcher>,
    #[cfg(test)]
    unanchored_prefix: Option<bool>,
}
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug, Default)]
pub struct Config {
    accelerate: Option<bool>,
    pre: Option<Option<Prefilter>>,
    minimize: Option<bool>,
    match_kind: Option<MatchKind>,
    start_kind: Option<StartKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    specialize_start_states: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    determinize_size_limit: Option<Option<usize>>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[derive(Clone)]
struct Transition {
    /// The byte range.
    range: Utf8Range,
    /// The next state to transition to.
    next_id: StateID,
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    crlf: bool,
    line_terminator: u8,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    utf8: bool,
    nest_limit: u32,
    octal: bool,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    specialize_start_states: Option<bool>,
    cache_capacity: Option<usize>,
    skip_cache_capacity_check: Option<bool>,
    minimum_cache_clear_count: Option<Option<usize>>,
    minimum_bytes_per_state: Option<Option<usize>>,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    utf8_empty: Option<bool>,
    autopre: Option<bool>,
    pre: Option<Option<Prefilter>>,
    which_captures: Option<WhichCaptures>,
    nfa_size_limit: Option<Option<usize>>,
    onepass_size_limit: Option<Option<usize>>,
    hybrid_cache_capacity: Option<usize>,
    hybrid: Option<bool>,
    dfa: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    dfa_state_limit: Option<Option<usize>>,
    onepass: Option<bool>,
    backtrack: Option<bool>,
    byte_classes: Option<bool>,
    line_terminator: Option<u8>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
impl core::fmt::Debug for DFA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        fn debug_state_transitions(
            f: &mut core::fmt::Formatter,
            dfa: &DFA,
            sid: StateID,
        ) -> core::fmt::Result {
            for (i, (start, end, trans)) in dfa.sparse_transitions(sid).enumerate() {
                let next = trans.state_id();
                if i > 0 {
                    write!(f, ", ")?;
                }
                if start == end {
                    write!(f, "{:?} => {:?}", DebugByte(start), next.as_usize(),)?;
                } else {
                    write!(
                        f, "{:?}-{:?} => {:?}", DebugByte(start), DebugByte(end), next
                        .as_usize(),
                    )?;
                }
                if trans.match_wins() {
                    write!(f, " (MW)")?;
                }
                if !trans.epsilons().is_empty() {
                    write!(f, " ({:?})", trans.epsilons())?;
                }
            }
            Ok(())
        }
        writeln!(f, "onepass::DFA(")?;
        for index in 0..self.state_len() {
            let sid = StateID::must(index);
            let pateps = self.pattern_epsilons(sid);
            if sid == DEAD {
                write!(f, "D ")?;
            } else if pateps.pattern_id().is_some() {
                write!(f, "* ")?;
            } else {
                write!(f, "  ")?;
            }
            write!(f, "{:06?}", sid.as_usize())?;
            if !pateps.is_empty() {
                write!(f, " ({:?})", pateps)?;
            }
            write!(f, ": ")?;
            debug_state_transitions(f, self, sid)?;
            write!(f, "\n")?;
        }
        writeln!(f, "")?;
        for (i, &sid) in self.starts.iter().enumerate() {
            if i == 0 {
                writeln!(f, "START(ALL): {:?}", sid.as_usize())?;
            } else {
                writeln!(f, "START(pattern: {:?}): {:?}", i - 1, sid.as_usize(),)?;
            }
        }
        writeln!(f, "state length: {:?}", self.state_len())?;
        writeln!(f, "pattern length: {:?}", self.pattern_len())?;
        writeln!(f, ")")?;
        Ok(())
    }
}
impl PatternEpsilons {
    const PATTERN_ID_BITS: u64 = 22;
    const PATTERN_ID_SHIFT: u64 = 64 - PatternEpsilons::PATTERN_ID_BITS;
    const PATTERN_ID_NONE: u64 = 0x00000000_003FFFFF;
    const PATTERN_ID_LIMIT: u64 = PatternEpsilons::PATTERN_ID_NONE;
    const PATTERN_ID_MASK: u64 = 0xFFFFFC00_00000000;
    const EPSILONS_MASK: u64 = 0x000003FF_FFFFFFFF;
    fn empty() -> PatternEpsilons {}
    fn is_empty(self) -> bool {
        self.pattern_id().is_none() && self.epsilons().is_empty()
    }
    fn pattern_id(self) -> Option<PatternID> {
        let pid = self.0 >> PatternEpsilons::PATTERN_ID_SHIFT;
        if pid == PatternEpsilons::PATTERN_ID_LIMIT {
            None
        } else {
            Some(PatternID::new_unchecked(pid.as_usize()))
        }
    }
    fn pattern_id_unchecked(self) -> PatternID {}
    fn set_pattern_id(self, pid: PatternID) -> PatternEpsilons {}
    fn epsilons(self) -> Epsilons {}
    fn set_epsilons(self, epsilons: Epsilons) -> PatternEpsilons {}
}
impl DFA {
    fn start(&self) -> StateID {}
    fn start_pattern(&self, pid: PatternID) -> Result<StateID, MatchError> {}
    fn transition(&self, sid: StateID, byte: u8) -> Transition {}
    fn set_transition(&mut self, sid: StateID, byte: u8, to: Transition) {}
    fn sparse_transitions(&self, sid: StateID) -> SparseTransitionIter<'_> {}
    fn pattern_epsilons(&self, sid: StateID) -> PatternEpsilons {
        let offset = sid.as_usize() << self.stride2();
        PatternEpsilons(self.table[offset + self.pateps_offset].0)
    }
    fn set_pattern_epsilons(&mut self, sid: StateID, pateps: PatternEpsilons) {}
    fn prev_state_id(&self, id: StateID) -> Option<StateID> {}
    fn last_state_id(&self) -> StateID {}
    pub(super) fn swap_states(&mut self, id1: StateID, id2: StateID) {}
    pub(super) fn remap(&mut self, map: impl Fn(StateID) -> StateID) {}
}
