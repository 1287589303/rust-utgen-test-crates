#[cfg(feature = "std")]
type StateMap = std::collections::HashMap<State, StateID>;
#[cfg(not(feature = "std"))]
type StateMap = BTreeMap<State, StateID>;
use alloc::{collections::BTreeMap, vec::Vec};
use crate::{
    dfa::{
        dense::{self, BuildError},
        DEAD,
    },
    nfa::thompson,
    util::{
        self, alphabet::{self, ByteSet},
        determinize::{State, StateBuilderEmpty, StateBuilderNFA},
        primitives::{PatternID, StateID},
        search::{Anchored, MatchKind},
        sparse_set::SparseSets, start::Start,
    },
};
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}
#[derive(Clone, Debug)]
pub(crate) struct StateBuilderEmpty(Vec<u8>);
#[derive(Debug)]
struct Runner<'a> {
    /// The configuration used to initialize determinization.
    config: Config,
    /// The NFA we're converting into a DFA.
    nfa: &'a thompson::NFA,
    /// The DFA we're building.
    dfa: &'a mut dense::OwnedDFA,
    /// Each DFA state being built is defined as an *ordered* set of NFA
    /// states, along with some meta facts about the ordered set of NFA states.
    ///
    /// This is never empty. The first state is always a dummy state such that
    /// a state id == 0 corresponds to a dead state. The second state is always
    /// the quit state.
    ///
    /// Why do we have states in both a `Vec` and in a cache map below?
    /// Well, they serve two different roles based on access patterns.
    /// `builder_states` is the canonical home of each state, and provides
    /// constant random access by a DFA state's ID. The cache map below, on
    /// the other hand, provides a quick way of searching for identical DFA
    /// states by using the DFA state as a key in the map. Of course, we use
    /// reference counting to avoid actually duplicating the state's data
    /// itself. (Although this has never been benchmarked.) Note that the cache
    /// map does not give us full minimization; it just lets us avoid some very
    /// obvious redundant states.
    ///
    /// Note that the index into this Vec isn't quite the DFA's state ID.
    /// Rather, it's just an index. To get the state ID, you have to multiply
    /// it by the DFA's stride. That's done by self.dfa.from_index. And the
    /// inverse is self.dfa.to_index.
    ///
    /// Moreover, DFA states don't usually retain the IDs assigned to them
    /// by their position in this Vec. After determinization completes,
    /// states are shuffled around to support other optimizations. See the
    /// sibling 'special' module for more details on that. (The reason for
    /// mentioning this is that if you print out the DFA for debugging during
    /// determinization, and then print out the final DFA after it is fully
    /// built, then the state IDs likely won't match up.)
    builder_states: Vec<State>,
    /// A cache of DFA states that already exist and can be easily looked up
    /// via ordered sets of NFA states.
    ///
    /// See `builder_states` docs for why we store states in two different
    /// ways.
    cache: StateMap,
    /// The memory usage, in bytes, used by builder_states and cache. We track
    /// this as new states are added since states use a variable amount of
    /// heap. Tracking this as we add states makes it possible to compute the
    /// total amount of memory used by the determinizer in constant time.
    memory_usage_state: usize,
    /// A pair of sparse sets for tracking ordered sets of NFA state IDs.
    /// These are reused throughout determinization. A bounded sparse set
    /// gives us constant time insertion, membership testing and clearing.
    sparses: SparseSets,
    /// Scratch space for a stack of NFA states to visit, for depth first
    /// visiting without recursion.
    stack: Vec<StateID>,
    /// Scratch space for storing an ordered sequence of NFA states, for
    /// amortizing allocation. This is principally useful for when we avoid
    /// adding a new DFA state since it already exists. In order to detect this
    /// case though, we still need an ordered set of NFA state IDs. So we use
    /// this space to stage that ordered set before we know whether we need to
    /// create a new DFA state or not.
    scratch_state_builder: StateBuilderEmpty,
}
#[derive(Clone, Debug)]
pub(crate) struct SparseSets {
    pub(crate) set1: SparseSet,
    pub(crate) set2: SparseSet,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct State(Arc<[u8]>);
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchKind {
    /// Report all possible matches.
    All,
    /// Report only the leftmost matches. When multiple leftmost matches exist,
    /// report the match corresponding to the part of the regex that appears
    /// first in the syntax.
    LeftmostFirst,
}
#[derive(Clone, Eq, PartialEq)]
pub enum State {
    /// A state with a single transition that can only be taken if the current
    /// input symbol is in a particular range of bytes.
    ByteRange {
        /// The transition from this state to the next.
        trans: Transition,
    },
    /// A state with possibly many transitions represented in a sparse fashion.
    /// Transitions are non-overlapping and ordered lexicographically by input
    /// range.
    ///
    /// In practice, this is used for encoding UTF-8 automata. Its presence is
    /// primarily an optimization that avoids many additional unconditional
    /// epsilon transitions (via [`Union`](State::Union) states), and thus
    /// decreases the overhead of traversing the NFA. This can improve both
    /// matching time and DFA construction time.
    Sparse(SparseTransitions),
    /// A dense representation of a state with multiple transitions.
    Dense(DenseTransitions),
    /// A conditional epsilon transition satisfied via some sort of
    /// look-around. Look-around is limited to anchor and word boundary
    /// assertions.
    ///
    /// Look-around states are meant to be evaluated while performing epsilon
    /// closure (computing the set of states reachable from a particular state
    /// via only epsilon transitions). If the current position in the haystack
    /// satisfies the look-around assertion, then you're permitted to follow
    /// that epsilon transition.
    Look {
        /// The look-around assertion that must be satisfied before moving
        /// to `next`.
        look: Look,
        /// The state to transition to if the look-around assertion is
        /// satisfied.
        next: StateID,
    },
    /// An alternation such that there exists an epsilon transition to all
    /// states in `alternates`, where matches found via earlier transitions
    /// are preferred over later transitions.
    Union {
        /// An ordered sequence of unconditional epsilon transitions to other
        /// states. Transitions earlier in the sequence are preferred over
        /// transitions later in the sequence.
        alternates: Box<[StateID]>,
    },
    /// An alternation such that there exists precisely two unconditional
    /// epsilon transitions, where matches found via `alt1` are preferred over
    /// matches found via `alt2`.
    ///
    /// This state exists as a common special case of Union where there are
    /// only two alternates. In this case, we don't need any allocations to
    /// represent the state. This saves a bit of memory and also saves an
    /// additional memory access when traversing the NFA.
    BinaryUnion {
        /// An unconditional epsilon transition to another NFA state. This
        /// is preferred over `alt2`.
        alt1: StateID,
        /// An unconditional epsilon transition to another NFA state. Matches
        /// reported via this transition should only be reported if no matches
        /// were found by following `alt1`.
        alt2: StateID,
    },
    /// An empty state that records a capture location.
    ///
    /// From the perspective of finite automata, this is precisely equivalent
    /// to an unconditional epsilon transition, but serves the purpose of
    /// instructing NFA simulations to record additional state when the finite
    /// state machine passes through this epsilon transition.
    ///
    /// `slot` in this context refers to the specific capture group slot
    /// offset that is being recorded. Each capturing group has two slots
    /// corresponding to the start and end of the matching portion of that
    /// group.
    ///
    /// The pattern ID and capture group index are also included in this state
    /// in case they are useful. But mostly, all you'll need is `next` and
    /// `slot`.
    Capture {
        /// The state to transition to, unconditionally.
        next: StateID,
        /// The pattern ID that this capture belongs to.
        pattern_id: PatternID,
        /// The capture group index that this capture belongs to. Capture group
        /// indices are local to each pattern. For example, when capturing
        /// groups are enabled, every pattern has a capture group at index
        /// `0`.
        group_index: SmallIndex,
        /// The slot index for this capture. Every capturing group has two
        /// slots: one for the start haystack offset and one for the end
        /// haystack offset. Unlike capture group indices, slot indices are
        /// global across all patterns in this NFA. That is, each slot belongs
        /// to a single pattern, but there is only one slot at index `i`.
        slot: SmallIndex,
    },
    /// A state that cannot be transitioned out of. This is useful for cases
    /// where you want to prevent matching from occurring. For example, if your
    /// regex parser permits empty character classes, then one could choose
    /// a `Fail` state to represent them. (An empty character class can be
    /// thought of as an empty set. Since nothing is in an empty set, they can
    /// never match anything.)
    Fail,
    /// A match state. There is at least one such occurrence of this state for
    /// each regex that can match that is in this NFA.
    Match {
        /// The matching pattern ID.
        pattern_id: PatternID,
    },
}
impl Config {
    pub fn new() -> Config {}
    pub fn run(
        &self,
        nfa: &thompson::NFA,
        dfa: &mut dense::OwnedDFA,
    ) -> Result<(), BuildError> {
        let dead = State::dead();
        let quit = State::dead();
        let mut cache = StateMap::default();
        cache.insert(dead.clone(), DEAD);
        let runner = Runner {
            config: self.clone(),
            nfa,
            dfa,
            builder_states: alloc::vec![dead, quit],
            cache,
            memory_usage_state: 0,
            sparses: SparseSets::new(nfa.states().len()),
            stack: alloc::vec![],
            scratch_state_builder: StateBuilderEmpty::new(),
        };
        runner.run()
    }
    pub fn match_kind(&mut self, kind: MatchKind) -> &mut Config {}
    pub fn quit(&mut self, set: ByteSet) -> &mut Config {}
    pub fn dfa_size_limit(&mut self, bytes: Option<usize>) -> &mut Config {}
    pub fn determinize_size_limit(&mut self, bytes: Option<usize>) -> &mut Config {}
}
impl StateBuilderEmpty {
    pub(crate) fn new() -> StateBuilderEmpty {
        StateBuilderEmpty(alloc::vec![])
    }
    pub(crate) fn into_matches(mut self) -> StateBuilderMatches {}
    fn clear(&mut self) {}
    pub(crate) fn capacity(&self) -> usize {}
}
impl<'a> Runner<'a> {
    fn run(mut self) -> Result<(), BuildError> {
        if self.nfa.look_set_any().contains_word_unicode()
            && !self.config.quit.contains_range(0x80, 0xFF)
        {
            return Err(BuildError::unsupported_dfa_word_boundary_unicode());
        }
        let representatives: Vec<alphabet::Unit> = self
            .dfa
            .byte_classes()
            .representatives(..)
            .collect();
        let mut uncompiled = alloc::vec![];
        self.add_all_starts(&mut uncompiled)?;
        while let Some(dfa_id) = uncompiled.pop() {
            for &unit in &representatives {
                if unit.as_u8().map_or(false, |b| self.config.quit.contains(b)) {
                    continue;
                }
                let (next_dfa_id, is_new) = self.cached_state(dfa_id, unit)?;
                self.dfa.set_transition(dfa_id, unit, next_dfa_id);
                if is_new {
                    uncompiled.push(next_dfa_id);
                }
            }
        }
        debug!(
            "determinization complete, memory usage: {}, \
             dense DFA size: {}, \
             is reverse? {}",
            self.memory_usage(), self.dfa.memory_usage(), self.nfa.is_reverse(),
        );
        let mut matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new();
        self.cache.clear();
        #[cfg(feature = "logging")]
        let mut total_pat_len = 0;
        for (i, state) in self.builder_states.into_iter().enumerate() {
            if let Some(pat_ids) = state.match_pattern_ids() {
                let id = self.dfa.to_state_id(i);
                log! {
                    total_pat_len += pat_ids.len();
                }
                matches.insert(id, pat_ids);
            }
        }
        log! {
            use core::mem::size_of; let per_elem = size_of::< StateID > () + size_of::<
            Vec < PatternID >> (); let pats = total_pat_len * size_of::< PatternID > ();
            let mem = (matches.len() * per_elem) + pats;
            log::debug!("matches map built, memory usage: {}", mem);
        }
        self.dfa.shuffle(matches)?;
        Ok(())
    }
    fn cached_state(
        &mut self,
        dfa_id: StateID,
        unit: alphabet::Unit,
    ) -> Result<(StateID, bool), BuildError> {}
    fn add_all_starts(
        &mut self,
        dfa_state_ids: &mut Vec<StateID>,
    ) -> Result<(), BuildError> {}
    fn add_start_group(
        &mut self,
        anchored: Anchored,
        dfa_state_ids: &mut Vec<StateID>,
    ) -> Result<(), BuildError> {}
    fn add_one_start(
        &mut self,
        nfa_start: StateID,
        start: Start,
    ) -> Result<(StateID, bool), BuildError> {}
    fn maybe_add_state(
        &mut self,
        builder: StateBuilderNFA,
    ) -> Result<(StateID, bool), BuildError> {}
    fn add_state(&mut self, builder: StateBuilderNFA) -> Result<StateID, BuildError> {}
    fn get_state_builder(&mut self) -> StateBuilderEmpty {}
    fn put_state_builder(&mut self, builder: StateBuilderNFA) {}
    fn memory_usage(&self) -> usize {}
}
impl SparseSets {
    pub(crate) fn new(capacity: usize) -> SparseSets {
        SparseSets {
            set1: SparseSet::new(capacity),
            set2: SparseSet::new(capacity),
        }
    }
    #[inline]
    pub(crate) fn resize(&mut self, new_capacity: usize) {}
    pub(crate) fn clear(&mut self) {}
    pub(crate) fn swap(&mut self) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl NFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<NFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError> {}
    pub fn always_match() -> NFA {}
    pub fn never_match() -> NFA {}
    #[cfg(feature = "syntax")]
    pub fn config() -> Config {}
    #[cfg(feature = "syntax")]
    pub fn compiler() -> Compiler {}
    pub fn patterns(&self) -> PatternIter<'_> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {}
    #[inline]
    pub fn start_anchored(&self) -> StateID {}
    #[inline]
    pub fn start_unanchored(&self) -> StateID {}
    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {}
    #[inline]
    pub(crate) fn byte_class_set(&self) -> &ByteClassSet {}
    #[inline]
    pub fn byte_classes(&self) -> &ByteClasses {}
    #[inline]
    pub fn state(&self, id: StateID) -> &State {}
    #[inline]
    pub fn states(&self) -> &[State] {
        &self.0.states
    }
    #[inline]
    pub fn group_info(&self) -> &GroupInfo {}
    #[inline]
    pub fn has_capture(&self) -> bool {}
    #[inline]
    pub fn has_empty(&self) -> bool {}
    #[inline]
    pub fn is_utf8(&self) -> bool {}
    #[inline]
    pub fn is_reverse(&self) -> bool {}
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {}
    #[inline]
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl State {
    pub(crate) fn dead() -> State {
        StateBuilderEmpty::new().into_matches().into_nfa().to_state()
    }
    pub(crate) fn is_match(&self) -> bool {}
    pub(crate) fn is_from_word(&self) -> bool {}
    pub(crate) fn is_half_crlf(&self) -> bool {}
    pub(crate) fn look_have(&self) -> LookSet {}
    pub(crate) fn look_need(&self) -> LookSet {}
    pub(crate) fn match_len(&self) -> usize {}
    pub(crate) fn match_pattern(&self, index: usize) -> PatternID {}
    pub(crate) fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    #[cfg(all(test, not(miri)))]
    pub(crate) fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, f: F) {}
    pub(crate) fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, f: F) {}
    pub(crate) fn memory_usage(&self) -> usize {}
    fn repr(&self) -> Repr<'_> {}
}
