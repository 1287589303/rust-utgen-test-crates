use core::{borrow::Borrow, cell::RefCell};
use alloc::{sync::Arc, vec, vec::Vec};
use regex_syntax::{
    hir::{self, Hir},
    utf8::{Utf8Range, Utf8Sequences},
    ParserBuilder,
};
use crate::{
    nfa::thompson::{
        builder::Builder, error::BuildError, literal_trie::LiteralTrie,
        map::{Utf8BoundedMap, Utf8SuffixKey, Utf8SuffixMap},
        nfa::{Transition, NFA},
        range_trie::RangeTrie,
    },
    util::{
        look::{Look, LookMatcher},
        primitives::{PatternID, StateID},
    },
};
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
    utf8: Option<bool>,
    reverse: Option<bool>,
    nfa_size_limit: Option<Option<usize>>,
    shrink: Option<bool>,
    which_captures: Option<WhichCaptures>,
    look_matcher: Option<LookMatcher>,
    #[cfg(test)]
    unanchored_prefix: Option<bool>,
}
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The ID of the pattern that we're currently building.
    ///
    /// Callers are required to set (and unset) this by calling
    /// {start,finish}_pattern. Otherwise, most methods will panic.
    pattern_id: Option<PatternID>,
    /// A sequence of intermediate NFA states. Once a state is added to this
    /// sequence, it is assigned a state ID equivalent to its index. Once a
    /// state is added, it is still expected to be mutated, e.g., to set its
    /// transition to a state that didn't exist at the time it was added.
    states: Vec<State>,
    /// The starting states for each individual pattern. Starting at any
    /// of these states will result in only an anchored search for the
    /// corresponding pattern. The vec is indexed by pattern ID. When the NFA
    /// contains a single regex, then `start_pattern[0]` and `start_anchored`
    /// are always equivalent.
    start_pattern: Vec<StateID>,
    /// A map from pattern ID to capture group index to name. (If no name
    /// exists, then a None entry is present. Thus, all capturing groups are
    /// present in this mapping.)
    ///
    /// The outer vec is indexed by pattern ID, while the inner vec is indexed
    /// by capture index offset for the corresponding pattern.
    ///
    /// The first capture group for each pattern is always unnamed and is thus
    /// always None.
    captures: Vec<Vec<Option<Arc<str>>>>,
    /// The combined memory used by each of the 'State's in 'states'. This
    /// only includes heap usage by each state, and not the size of the state
    /// itself. In other words, this tracks heap memory used that isn't
    /// captured via `size_of::<State>() * states.len()`.
    memory_states: usize,
    /// Whether this NFA only matches UTF-8 and whether regex engines using
    /// this NFA for searching should report empty matches that split a
    /// codepoint.
    utf8: bool,
    /// Whether this NFA should be matched in reverse or not.
    reverse: bool,
    /// The matcher to use for look-around assertions.
    look_matcher: LookMatcher,
    /// A size limit to respect when building an NFA. If the total heap memory
    /// of the intermediate NFA states exceeds (or would exceed) this amount,
    /// then an error is returned.
    size_limit: Option<usize>,
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
pub struct Builder {
    #[cfg(feature = "dfa-build")]
    dfa: dense::Builder,
}
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone)]
pub struct RangeTrie {
    /// The states in this trie. The first is always the shared final state.
    /// The second is always the root state. Otherwise, there is no
    /// particular order.
    states: Vec<State>,
    /// A free-list of states. When a range trie is cleared, all of its states
    /// are added to this list. Creating a new state reuses states from this
    /// list before allocating a new one.
    free: Vec<State>,
    /// A stack for traversing this trie to yield sequences of byte ranges in
    /// lexicographic order.
    iter_stack: RefCell<Vec<NextIter>>,
    /// A buffer that stores the current sequence during iteration.
    iter_ranges: RefCell<Vec<Utf8Range>>,
    /// A stack used for traversing the trie in order to (deeply) duplicate
    /// a state. States are recursively duplicated when ranges are split.
    dupe_stack: Vec<NextDupe>,
    /// A stack used for traversing the trie during insertion of a new
    /// sequence of byte ranges.
    insert_stack: Vec<NextInsert>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[derive(Clone, Debug)]
pub struct Builder {
    dfa: dfa::Builder,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct ThompsonRef {
    pub(crate) start: StateID,
    pub(crate) end: StateID,
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
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}
#[derive(Clone, Debug)]
struct Utf8State {
    compiled: Utf8BoundedMap,
    uncompiled: Vec<Utf8Node>,
}
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
#[derive(Clone, Debug)]
pub struct LookMatcher {
    lineterm: DebugByte,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
#[derive(Clone, Debug)]
pub struct Utf8SuffixMap {
    /// The current version of this map. Only entries with matching versions
    /// are considered during lookups. If an entry is found with a mismatched
    /// version, then the map behaves as if the entry does not exist.
    version: u16,
    /// The total number of entries this map can store.
    capacity: usize,
    /// The actual entries, keyed by hash. Collisions between different states
    /// result in the old state being dropped.
    map: Vec<Utf8SuffixEntry>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
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
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Copy, Debug)]
pub enum WhichCaptures {
    /// All capture states, including those corresponding to both implicit and
    /// explicit capture groups, are included in the Thompson NFA.
    All,
    /// Only capture states corresponding to implicit capture groups are
    /// included. Implicit capture groups appear in every pattern implicitly
    /// and correspond to the overall match of a pattern.
    ///
    /// This is useful when one only cares about the overall match of a
    /// pattern. By excluding capture states from explicit capture groups,
    /// one might be able to reduce the memory usage of a multi-pattern regex
    /// substantially if it was otherwise written to have many explicit capture
    /// groups.
    Implicit,
    /// No capture states are compiled into the Thompson NFA.
    ///
    /// This is useful when capture states are either not needed (for example,
    /// if one is only trying to build a DFA) or if they aren't supported (for
    /// example, a reverse NFA).
    None,
}
impl Compiler {
    fn compile<H: Borrow<Hir>>(&self, exprs: &[H]) -> Result<NFA, BuildError> {
        if exprs.len() > PatternID::LIMIT {
            return Err(BuildError::too_many_patterns(exprs.len()));
        }
        if self.config.get_reverse() && self.config.get_which_captures().is_any() {
            return Err(BuildError::unsupported_captures());
        }
        self.builder.borrow_mut().clear();
        self.builder.borrow_mut().set_utf8(self.config.get_utf8());
        self.builder.borrow_mut().set_reverse(self.config.get_reverse());
        self.builder.borrow_mut().set_look_matcher(self.config.get_look_matcher());
        self.builder.borrow_mut().set_size_limit(self.config.get_nfa_size_limit())?;
        let all_anchored = exprs
            .iter()
            .all(|e| {
                let props = e.borrow().properties();
                if self.config.get_reverse() {
                    props.look_set_suffix().contains(hir::Look::End)
                } else {
                    props.look_set_prefix().contains(hir::Look::Start)
                }
            });
        let anchored = !self.config.get_unanchored_prefix() || all_anchored;
        let unanchored_prefix = if anchored {
            self.c_empty()?
        } else {
            self.c_at_least(&Hir::dot(hir::Dot::AnyByte), false, 0)?
        };
        let compiled = self
            .c_alt_iter(
                exprs
                    .iter()
                    .map(|e| {
                        let _ = self.start_pattern()?;
                        let one = self.c_cap(0, None, e.borrow())?;
                        let match_state_id = self.add_match()?;
                        self.patch(one.end, match_state_id)?;
                        let _ = self.finish_pattern(one.start)?;
                        Ok(ThompsonRef {
                            start: one.start,
                            end: match_state_id,
                        })
                    }),
            )?;
        self.patch(unanchored_prefix.end, compiled.start)?;
        let nfa = self
            .builder
            .borrow_mut()
            .build(compiled.start, unanchored_prefix.start)?;
        debug!("HIR-to-NFA compilation complete, config: {:?}", self.config);
        Ok(nfa)
    }
    fn c(&self, expr: &Hir) -> Result<ThompsonRef, BuildError> {}
    fn c_concat<I>(&self, mut it: I) -> Result<ThompsonRef, BuildError>
    where
        I: DoubleEndedIterator<Item = Result<ThompsonRef, BuildError>>,
    {}
    fn c_alt_slice(&self, exprs: &[Hir]) -> Result<ThompsonRef, BuildError> {}
    fn c_alt_iter<I>(&self, mut it: I) -> Result<ThompsonRef, BuildError>
    where
        I: Iterator<Item = Result<ThompsonRef, BuildError>>,
    {}
    fn c_cap(
        &self,
        index: u32,
        name: Option<&str>,
        expr: &Hir,
    ) -> Result<ThompsonRef, BuildError> {}
    fn c_repetition(&self, rep: &hir::Repetition) -> Result<ThompsonRef, BuildError> {}
    fn c_bounded(
        &self,
        expr: &Hir,
        greedy: bool,
        min: u32,
        max: u32,
    ) -> Result<ThompsonRef, BuildError> {}
    fn c_at_least(
        &self,
        expr: &Hir,
        greedy: bool,
        n: u32,
    ) -> Result<ThompsonRef, BuildError> {
        if n == 0 {
            if expr.properties().minimum_len().map_or(false, |len| len > 0) {
                let union = if greedy {
                    self.add_union()
                } else {
                    self.add_union_reverse()
                }?;
                let compiled = self.c(expr)?;
                self.patch(union, compiled.start)?;
                self.patch(compiled.end, union)?;
                return Ok(ThompsonRef {
                    start: union,
                    end: union,
                });
            }
            let compiled = self.c(expr)?;
            let plus = if greedy { self.add_union() } else { self.add_union_reverse() }?;
            self.patch(compiled.end, plus)?;
            self.patch(plus, compiled.start)?;
            let question = if greedy {
                self.add_union()
            } else {
                self.add_union_reverse()
            }?;
            let empty = self.add_empty()?;
            self.patch(question, compiled.start)?;
            self.patch(question, empty)?;
            self.patch(plus, empty)?;
            Ok(ThompsonRef {
                start: question,
                end: empty,
            })
        } else if n == 1 {
            let compiled = self.c(expr)?;
            let union = if greedy {
                self.add_union()
            } else {
                self.add_union_reverse()
            }?;
            self.patch(compiled.end, union)?;
            self.patch(union, compiled.start)?;
            Ok(ThompsonRef {
                start: compiled.start,
                end: union,
            })
        } else {
            let prefix = self.c_exactly(expr, n - 1)?;
            let last = self.c(expr)?;
            let union = if greedy {
                self.add_union()
            } else {
                self.add_union_reverse()
            }?;
            self.patch(prefix.end, last.start)?;
            self.patch(last.end, union)?;
            self.patch(union, last.start)?;
            Ok(ThompsonRef {
                start: prefix.start,
                end: union,
            })
        }
    }
    fn c_zero_or_one(
        &self,
        expr: &Hir,
        greedy: bool,
    ) -> Result<ThompsonRef, BuildError> {}
    fn c_exactly(&self, expr: &Hir, n: u32) -> Result<ThompsonRef, BuildError> {}
    fn c_byte_class(&self, cls: &hir::ClassBytes) -> Result<ThompsonRef, BuildError> {}
    fn c_unicode_class(
        &self,
        cls: &hir::ClassUnicode,
    ) -> Result<ThompsonRef, BuildError> {}
    fn c_unicode_class_reverse_with_suffix(
        &self,
        cls: &hir::ClassUnicode,
    ) -> Result<ThompsonRef, BuildError> {}
    fn c_look(&self, anchor: &hir::Look) -> Result<ThompsonRef, BuildError> {}
    fn c_literal(&self, bytes: &[u8]) -> Result<ThompsonRef, BuildError> {}
    fn c_range(&self, start: u8, end: u8) -> Result<ThompsonRef, BuildError> {}
    fn c_empty(&self) -> Result<ThompsonRef, BuildError> {
        let id = self.add_empty()?;
        Ok(ThompsonRef { start: id, end: id })
    }
    fn c_fail(&self) -> Result<ThompsonRef, BuildError> {}
    fn patch(&self, from: StateID, to: StateID) -> Result<(), BuildError> {
        self.builder.borrow_mut().patch(from, to)
    }
    fn start_pattern(&self) -> Result<PatternID, BuildError> {}
    fn finish_pattern(&self, start_id: StateID) -> Result<PatternID, BuildError> {}
    fn add_empty(&self) -> Result<StateID, BuildError> {}
    fn add_range(&self, start: u8, end: u8) -> Result<StateID, BuildError> {}
    fn add_sparse(&self, ranges: Vec<Transition>) -> Result<StateID, BuildError> {}
    fn add_look(&self, mut look: Look) -> Result<StateID, BuildError> {}
    fn add_union(&self) -> Result<StateID, BuildError> {}
    fn add_union_reverse(&self) -> Result<StateID, BuildError> {}
    fn add_capture_start(
        &self,
        capture_index: u32,
        name: Option<&str>,
    ) -> Result<StateID, BuildError> {}
    fn add_capture_end(&self, capture_index: u32) -> Result<StateID, BuildError> {}
    fn add_fail(&self) -> Result<StateID, BuildError> {}
    fn add_match(&self) -> Result<StateID, BuildError> {}
    fn is_reverse(&self) -> bool {}
}
impl Config {
    pub fn new() -> Config {}
    pub fn utf8(mut self, yes: bool) -> Config {}
    pub fn reverse(mut self, yes: bool) -> Config {}
    pub fn nfa_size_limit(mut self, bytes: Option<usize>) -> Config {}
    pub fn shrink(mut self, yes: bool) -> Config {}
    #[deprecated(since = "0.3.5", note = "use which_captures instead")]
    pub fn captures(self, yes: bool) -> Config {}
    pub fn which_captures(mut self, which_captures: WhichCaptures) -> Config {}
    pub fn look_matcher(mut self, m: LookMatcher) -> Config {}
    #[cfg(test)]
    fn unanchored_prefix(mut self, yes: bool) -> Config {}
    pub fn get_utf8(&self) -> bool {
        self.utf8.unwrap_or(true)
    }
    pub fn get_reverse(&self) -> bool {
        self.reverse.unwrap_or(false)
    }
    pub fn get_nfa_size_limit(&self) -> Option<usize> {
        self.nfa_size_limit.unwrap_or(None)
    }
    pub fn get_shrink(&self) -> bool {}
    #[deprecated(since = "0.3.5", note = "use get_which_captures instead")]
    pub fn get_captures(&self) -> bool {}
    pub fn get_which_captures(&self) -> WhichCaptures {
        self.which_captures.unwrap_or(WhichCaptures::All)
    }
    pub fn get_look_matcher(&self) -> LookMatcher {
        self.look_matcher.clone().unwrap_or(LookMatcher::default())
    }
    fn get_unanchored_prefix(&self) -> bool {
        #[cfg(test)] { self.unanchored_prefix.unwrap_or(true) }
        #[cfg(not(test))] { true }
    }
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
impl BuildError {
    pub fn size_limit(&self) -> Option<usize> {}
    fn kind(&self) -> &BuildErrorKind {}
    #[cfg(feature = "syntax")]
    pub(crate) fn syntax(err: regex_syntax::Error) -> BuildError {}
    pub(crate) fn captures(err: captures::GroupInfoError) -> BuildError {}
    pub(crate) fn word(err: look::UnicodeWordBoundaryError) -> BuildError {}
    pub(crate) fn too_many_patterns(given: usize) -> BuildError {
        let limit = PatternID::LIMIT;
        BuildError {
            kind: BuildErrorKind::TooManyPatterns {
                given,
                limit,
            },
        }
    }
    pub(crate) fn too_many_states(given: usize) -> BuildError {}
    pub(crate) fn exceeded_size_limit(limit: usize) -> BuildError {}
    pub(crate) fn invalid_capture_index(index: u32) -> BuildError {}
    #[cfg(feature = "syntax")]
    pub(crate) fn unsupported_captures() -> BuildError {
        BuildError {
            kind: BuildErrorKind::UnsupportedCaptures,
        }
    }
}
impl WhichCaptures {
    pub fn is_none(&self) -> bool {}
    pub fn is_any(&self) -> bool {
        !self.is_none()
    }
}
impl Builder {
    pub fn new() -> Builder {}
    pub fn clear(&mut self) {
        self.pattern_id = None;
        self.states.clear();
        self.start_pattern.clear();
        self.captures.clear();
        self.memory_states = 0;
    }
    pub fn build(
        &self,
        start_anchored: StateID,
        start_unanchored: StateID,
    ) -> Result<NFA, BuildError> {
        assert!(self.pattern_id.is_none(), "must call 'finish_pattern' first");
        debug!(
            "intermediate NFA compilation via builder is complete, \
             intermediate NFA size: {} states, {} bytes on heap",
            self.states.len(), self.memory_usage(),
        );
        let mut nfa = nfa::Inner::default();
        nfa.set_utf8(self.utf8);
        nfa.set_reverse(self.reverse);
        nfa.set_look_matcher(self.look_matcher.clone());
        let mut empties = vec![];
        let mut remap = vec![];
        remap.resize(self.states.len(), StateID::ZERO);
        nfa.set_starts(start_anchored, start_unanchored, &self.start_pattern);
        nfa.set_captures(&self.captures).map_err(BuildError::captures)?;
        for (sid, state) in self.states.iter().with_state_ids() {
            match *state {
                State::Empty { next } => {
                    empties.push((sid, next));
                }
                State::ByteRange { trans } => {
                    remap[sid] = nfa.add(nfa::State::ByteRange { trans });
                }
                State::Sparse { ref transitions } => {
                    remap[sid] = match transitions.len() {
                        0 => nfa.add(nfa::State::Fail),
                        1 => {
                            nfa.add(nfa::State::ByteRange {
                                trans: transitions[0],
                            })
                        }
                        _ => {
                            let transitions = transitions.to_vec().into_boxed_slice();
                            let sparse = SparseTransitions { transitions };
                            nfa.add(nfa::State::Sparse(sparse))
                        }
                    };
                }
                State::Look { look, next } => {
                    remap[sid] = nfa.add(nfa::State::Look { look, next });
                }
                State::CaptureStart { pattern_id, group_index, next } => {
                    let slot = nfa
                        .group_info()
                        .slot(pattern_id, group_index.as_usize())
                        .expect("invalid capture index");
                    let slot = SmallIndex::new(slot).expect("a small enough slot");
                    remap[sid] = nfa
                        .add(nfa::State::Capture {
                            next,
                            pattern_id,
                            group_index,
                            slot,
                        });
                }
                State::CaptureEnd { pattern_id, group_index, next } => {
                    let slot = nfa
                        .group_info()
                        .slot(pattern_id, group_index.as_usize())
                        .expect("invalid capture index")
                        .checked_add(1)
                        .unwrap();
                    let slot = SmallIndex::new(slot).expect("a small enough slot");
                    remap[sid] = nfa
                        .add(nfa::State::Capture {
                            next,
                            pattern_id,
                            group_index,
                            slot,
                        });
                }
                State::Union { ref alternates } => {
                    if alternates.is_empty() {
                        remap[sid] = nfa.add(nfa::State::Fail);
                    } else if alternates.len() == 1 {
                        empties.push((sid, alternates[0]));
                        remap[sid] = alternates[0];
                    } else if alternates.len() == 2 {
                        remap[sid] = nfa
                            .add(nfa::State::BinaryUnion {
                                alt1: alternates[0],
                                alt2: alternates[1],
                            });
                    } else {
                        let alternates = alternates.to_vec().into_boxed_slice();
                        remap[sid] = nfa.add(nfa::State::Union { alternates });
                    }
                }
                State::UnionReverse { ref alternates } => {
                    if alternates.is_empty() {
                        remap[sid] = nfa.add(nfa::State::Fail);
                    } else if alternates.len() == 1 {
                        empties.push((sid, alternates[0]));
                        remap[sid] = alternates[0];
                    } else if alternates.len() == 2 {
                        remap[sid] = nfa
                            .add(nfa::State::BinaryUnion {
                                alt1: alternates[1],
                                alt2: alternates[0],
                            });
                    } else {
                        let mut alternates = alternates.to_vec().into_boxed_slice();
                        alternates.reverse();
                        remap[sid] = nfa.add(nfa::State::Union { alternates });
                    }
                }
                State::Fail => {
                    remap[sid] = nfa.add(nfa::State::Fail);
                }
                State::Match { pattern_id } => {
                    remap[sid] = nfa.add(nfa::State::Match { pattern_id });
                }
            }
        }
        let mut remapped = vec![false; self.states.len()];
        for &(empty_id, empty_next) in empties.iter() {
            if remapped[empty_id] {
                continue;
            }
            let mut new_next = empty_next;
            while let Some(next) = self.states[new_next].goto() {
                new_next = next;
            }
            remap[empty_id] = remap[new_next];
            remapped[empty_id] = true;
            let mut next2 = empty_next;
            while let Some(next) = self.states[next2].goto() {
                remap[next2] = remap[new_next];
                remapped[next2] = true;
                next2 = next;
            }
        }
        nfa.remap(&remap);
        let final_nfa = nfa.into_nfa();
        debug!(
            "NFA compilation via builder complete, \
             final NFA size: {} states, {} bytes on heap, \
             has empty? {:?}, utf8? {:?}",
            final_nfa.states().len(), final_nfa.memory_usage(), final_nfa.has_empty(),
            final_nfa.is_utf8(),
        );
        Ok(final_nfa)
    }
    pub fn start_pattern(&mut self) -> Result<PatternID, BuildError> {}
    pub fn finish_pattern(
        &mut self,
        start_id: StateID,
    ) -> Result<PatternID, BuildError> {}
    pub fn current_pattern_id(&self) -> PatternID {}
    pub fn pattern_len(&self) -> usize {}
    pub fn add_empty(&mut self) -> Result<StateID, BuildError> {}
    pub fn add_union(
        &mut self,
        alternates: Vec<StateID>,
    ) -> Result<StateID, BuildError> {}
    pub fn add_union_reverse(
        &mut self,
        alternates: Vec<StateID>,
    ) -> Result<StateID, BuildError> {}
    pub fn add_range(&mut self, trans: Transition) -> Result<StateID, BuildError> {}
    pub fn add_sparse(
        &mut self,
        transitions: Vec<Transition>,
    ) -> Result<StateID, BuildError> {}
    pub fn add_look(
        &mut self,
        next: StateID,
        look: Look,
    ) -> Result<StateID, BuildError> {}
    pub fn add_capture_start(
        &mut self,
        next: StateID,
        group_index: u32,
        name: Option<Arc<str>>,
    ) -> Result<StateID, BuildError> {}
    pub fn add_capture_end(
        &mut self,
        next: StateID,
        group_index: u32,
    ) -> Result<StateID, BuildError> {}
    pub fn add_fail(&mut self) -> Result<StateID, BuildError> {}
    pub fn add_match(&mut self) -> Result<StateID, BuildError> {}
    fn add(&mut self, state: State) -> Result<StateID, BuildError> {}
    pub fn patch(&mut self, from: StateID, to: StateID) -> Result<(), BuildError> {}
    pub fn set_utf8(&mut self, yes: bool) {
        self.utf8 = yes;
    }
    pub fn get_utf8(&self) -> bool {}
    pub fn set_reverse(&mut self, yes: bool) {
        self.reverse = yes;
    }
    pub fn get_reverse(&self) -> bool {}
    pub fn set_look_matcher(&mut self, m: LookMatcher) {
        self.look_matcher = m;
    }
    pub fn get_look_matcher(&self) -> &LookMatcher {}
    pub fn set_size_limit(&mut self, limit: Option<usize>) -> Result<(), BuildError> {
        self.size_limit = limit;
        self.check_size_limit()
    }
    pub fn get_size_limit(&self) -> Option<usize> {}
    pub fn memory_usage(&self) -> usize {}
    fn check_size_limit(&self) -> Result<(), BuildError> {}
}
