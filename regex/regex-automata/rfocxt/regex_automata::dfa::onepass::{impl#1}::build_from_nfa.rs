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
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Debug)]
struct InternalBuilder<'a> {
    /// The DFA we're building.
    dfa: DFA,
    /// An unordered collection of NFA state IDs that we haven't yet tried to
    /// build into a DFA state yet.
    ///
    /// This collection does not ultimately wind up including every NFA state
    /// ID. Instead, each ID represents a "start" state for a sub-graph of the
    /// NFA. The set of NFA states we then use to build a DFA state consists
    /// of that "start" state and all states reachable from it via epsilon
    /// transitions.
    uncompiled_nfa_ids: Vec<StateID>,
    /// A map from NFA state ID to DFA state ID. This is useful for easily
    /// determining whether an NFA state has been used as a "starting" point
    /// to build a DFA state yet. If it hasn't, then it is mapped to DEAD,
    /// and since DEAD is specially added and never corresponds to any NFA
    /// state, it follows that a mapping to DEAD implies the NFA state has
    /// no corresponding DFA state yet.
    nfa_to_dfa_id: Vec<StateID>,
    /// A stack used to traverse the NFA states that make up a single DFA
    /// state. Traversal occurs until the stack is empty, and we only push to
    /// the stack when the state ID isn't in 'seen'. Actually, even more than
    /// that, if we try to push something on to this stack that is already in
    /// 'seen', then we bail out on construction completely, since it implies
    /// that the NFA is not one-pass.
    stack: Vec<(StateID, Epsilons)>,
    /// The set of NFA states that we've visited via 'stack'.
    seen: SparseSet,
    /// Whether a match NFA state has been observed while constructing a
    /// one-pass DFA state. Once a match state is seen, assuming we are using
    /// leftmost-first match semantics, then we don't add any more transitions
    /// to the DFA state we're building.
    matched: bool,
    /// The config passed to the builder.
    ///
    /// This is duplicated in dfa.config.
    config: Config,
    /// The NFA we're building a one-pass DFA from.
    ///
    /// This is duplicated in dfa.nfa.
    nfa: &'a NFA,
    /// The equivalence classes that make up the alphabet for this DFA>
    ///
    /// This is duplicated in dfa.classes.
    classes: ByteClasses,
}
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
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
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
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug)]
pub struct Compiler {
    /// A regex parser, used when compiling an NFA directly from a pattern
    /// string.
    parser: ParserBuilder,
    /// The compiler configuration.
    config: Config,
    /// The builder for actually constructing an NFA. This provides a
    /// convenient abstraction for writing a compiler.
    builder: RefCell<Builder>,
    /// State used for compiling character classes to UTF-8 byte automata.
    /// State is not retained between character class compilations. This just
    /// serves to amortize allocation to the extent possible.
    utf8_state: RefCell<Utf8State>,
    /// State used for arranging character classes in reverse into a trie.
    trie_state: RefCell<RangeTrie>,
    /// State used for caching common suffixes when compiling reverse UTF-8
    /// automata (for Unicode character classes).
    utf8_suffix: RefCell<Utf8SuffixMap>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
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
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
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
impl Builder {
    pub fn new() -> Builder {}
    #[cfg(feature = "syntax")]
    pub fn build(&self, pattern: &str) -> Result<DFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError> {}
    pub fn build_from_nfa(&self, nfa: NFA) -> Result<DFA, BuildError> {
        InternalBuilder::new(self.config.clone(), &nfa).build()
    }
    pub fn configure(&mut self, config: Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn thompson(&mut self, config: thompson::Config) -> &mut Builder {}
}
impl<'a> InternalBuilder<'a> {
    fn new(config: Config, nfa: &'a NFA) -> InternalBuilder<'a> {
        let classes = if !config.get_byte_classes() {
            ByteClasses::singletons()
        } else {
            nfa.byte_classes().clone()
        };
        let alphabet_len = classes.alphabet_len().checked_sub(1).unwrap();
        let stride2 = classes.stride2();
        let dfa = DFA {
            config: config.clone(),
            nfa: nfa.clone(),
            table: vec![],
            starts: vec![],
            min_match_id: StateID::MAX,
            classes: classes.clone(),
            alphabet_len,
            stride2,
            pateps_offset: alphabet_len,
            explicit_slot_start: nfa.pattern_len().checked_mul(2).unwrap(),
        };
        InternalBuilder {
            dfa,
            uncompiled_nfa_ids: vec![],
            nfa_to_dfa_id: vec![DEAD; nfa.states().len()],
            stack: vec![],
            seen: SparseSet::new(nfa.states().len()),
            matched: false,
            config,
            nfa,
            classes,
        }
    }
    fn build(mut self) -> Result<DFA, BuildError> {
        self.nfa.look_set_any().available().map_err(BuildError::word)?;
        for look in self.nfa.look_set_any().iter() {
            if look.as_repr() > Look::WordUnicodeNegate.as_repr() {
                return Err(BuildError::unsupported_look(look));
            }
        }
        if self.nfa.pattern_len().as_u64() > PatternEpsilons::PATTERN_ID_LIMIT {
            return Err(BuildError::too_many_patterns(PatternEpsilons::PATTERN_ID_LIMIT));
        }
        if self.nfa.group_info().explicit_slot_len() > Slots::LIMIT {
            return Err(
                BuildError::not_one_pass(
                    "too many explicit capturing groups (max is 16)",
                ),
            );
        }
        assert_eq!(DEAD, self.add_empty_state() ?);
        let explicit_slot_start = self.nfa.pattern_len() * 2;
        self.add_start_state(None, self.nfa.start_anchored())?;
        if self.config.get_starts_for_each_pattern() {
            for pid in self.nfa.patterns() {
                self.add_start_state(Some(pid), self.nfa.start_pattern(pid).unwrap())?;
            }
        }
        while let Some(nfa_id) = self.uncompiled_nfa_ids.pop() {
            let dfa_id = self.nfa_to_dfa_id[nfa_id];
            self.matched = false;
            self.seen.clear();
            self.stack_push(nfa_id, Epsilons::empty())?;
            while let Some((id, epsilons)) = self.stack.pop() {
                match *self.nfa.state(id) {
                    thompson::State::ByteRange { ref trans } => {
                        self.compile_transition(dfa_id, trans, epsilons)?;
                    }
                    thompson::State::Sparse(ref sparse) => {
                        for trans in sparse.transitions.iter() {
                            self.compile_transition(dfa_id, trans, epsilons)?;
                        }
                    }
                    thompson::State::Dense(ref dense) => {
                        for trans in dense.iter() {
                            self.compile_transition(dfa_id, &trans, epsilons)?;
                        }
                    }
                    thompson::State::Look { look, next } => {
                        let looks = epsilons.looks().insert(look);
                        self.stack_push(next, epsilons.set_looks(looks))?;
                    }
                    thompson::State::Union { ref alternates } => {
                        for &sid in alternates.iter().rev() {
                            self.stack_push(sid, epsilons)?;
                        }
                    }
                    thompson::State::BinaryUnion { alt1, alt2 } => {
                        self.stack_push(alt2, epsilons)?;
                        self.stack_push(alt1, epsilons)?;
                    }
                    thompson::State::Capture { next, slot, .. } => {
                        let slot = slot.as_usize();
                        let epsilons = if slot < explicit_slot_start {
                            epsilons
                        } else {
                            let offset = slot - explicit_slot_start;
                            epsilons.set_slots(epsilons.slots().insert(offset))
                        };
                        self.stack_push(next, epsilons)?;
                    }
                    thompson::State::Fail => {
                        continue;
                    }
                    thompson::State::Match { pattern_id } => {
                        if self.matched {
                            return Err(
                                BuildError::not_one_pass(
                                    "multiple epsilon transitions to match state",
                                ),
                            );
                        }
                        self.matched = true;
                        self.dfa
                            .set_pattern_epsilons(
                                dfa_id,
                                PatternEpsilons::empty()
                                    .set_pattern_id(pattern_id)
                                    .set_epsilons(epsilons),
                            );
                    }
                }
            }
        }
        self.shuffle_states();
        Ok(self.dfa)
    }
    fn shuffle_states(&mut self) {}
    fn compile_transition(
        &mut self,
        dfa_id: StateID,
        trans: &thompson::Transition,
        epsilons: Epsilons,
    ) -> Result<(), BuildError> {}
    fn add_start_state(
        &mut self,
        pid: Option<PatternID>,
        nfa_id: StateID,
    ) -> Result<StateID, BuildError> {}
    fn add_dfa_state_for_nfa_state(
        &mut self,
        nfa_id: StateID,
    ) -> Result<StateID, BuildError> {}
    fn add_empty_state(&mut self) -> Result<StateID, BuildError> {}
    fn stack_push(
        &mut self,
        nfa_id: StateID,
        epsilons: Epsilons,
    ) -> Result<(), BuildError> {}
}
