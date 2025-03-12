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
#[derive(Clone, Debug)]
struct Utf8State {
    compiled: Utf8BoundedMap,
    uncompiled: Vec<Utf8Node>,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Utf8SuffixKey {
    pub from: StateID,
    pub start: u8,
    pub end: u8,
}
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
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
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone, Debug)]
pub struct Builder {
    #[cfg(feature = "dfa-build")]
    dfa: dense::Builder,
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
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct ThompsonRef {
    pub(crate) start: StateID,
    pub(crate) end: StateID,
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
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
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
pub struct Builder {
    dfa: dfa::Builder,
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
impl Compiler {
    fn compile<H: Borrow<Hir>>(&self, exprs: &[H]) -> Result<NFA, BuildError> {}
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
    ) -> Result<ThompsonRef, BuildError> {}
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
    ) -> Result<ThompsonRef, BuildError> {
        let mut cache = self.utf8_suffix.borrow_mut();
        cache.clear();
        let union = self.add_union()?;
        let alt_end = self.add_empty()?;
        for urng in cls.iter() {
            for seq in Utf8Sequences::new(urng.start(), urng.end()) {
                let mut end = alt_end;
                for brng in seq.as_slice() {
                    let key = Utf8SuffixKey {
                        from: end,
                        start: brng.start,
                        end: brng.end,
                    };
                    let hash = cache.hash(&key);
                    if let Some(id) = cache.get(&key, hash) {
                        end = id;
                        continue;
                    }
                    let compiled = self.c_range(brng.start, brng.end)?;
                    self.patch(compiled.end, end)?;
                    end = compiled.start;
                    cache.set(key, hash, end);
                }
                self.patch(union, end)?;
            }
        }
        Ok(ThompsonRef {
            start: union,
            end: alt_end,
        })
    }
    fn c_look(&self, anchor: &hir::Look) -> Result<ThompsonRef, BuildError> {}
    fn c_literal(&self, bytes: &[u8]) -> Result<ThompsonRef, BuildError> {}
    fn c_range(&self, start: u8, end: u8) -> Result<ThompsonRef, BuildError> {
        let id = self.add_range(start, end)?;
        Ok(ThompsonRef { start: id, end: id })
    }
    fn c_empty(&self) -> Result<ThompsonRef, BuildError> {}
    fn c_fail(&self) -> Result<ThompsonRef, BuildError> {}
    fn patch(&self, from: StateID, to: StateID) -> Result<(), BuildError> {
        self.builder.borrow_mut().patch(from, to)
    }
    fn start_pattern(&self) -> Result<PatternID, BuildError> {}
    fn finish_pattern(&self, start_id: StateID) -> Result<PatternID, BuildError> {}
    fn add_empty(&self) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_empty()
    }
    fn add_range(&self, start: u8, end: u8) -> Result<StateID, BuildError> {}
    fn add_sparse(&self, ranges: Vec<Transition>) -> Result<StateID, BuildError> {}
    fn add_look(&self, mut look: Look) -> Result<StateID, BuildError> {}
    fn add_union(&self) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_union(vec![])
    }
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
impl Utf8SuffixMap {
    pub fn new(capacity: usize) -> Utf8SuffixMap {}
    pub fn clear(&mut self) {
        if self.map.is_empty() {
            self.map = vec![Utf8SuffixEntry::default(); self.capacity];
        } else {
            self.version = self.version.wrapping_add(1);
            if self.version == 0 {
                self.map = vec![Utf8SuffixEntry::default(); self.capacity];
            }
        }
    }
    pub fn hash(&self, key: &Utf8SuffixKey) -> usize {
        const PRIME: u64 = 1099511628211;
        const INIT: u64 = 14695981039346656037;
        let mut h = INIT;
        h = (h ^ key.from.as_u64()).wrapping_mul(PRIME);
        h = (h ^ u64::from(key.start)).wrapping_mul(PRIME);
        h = (h ^ u64::from(key.end)).wrapping_mul(PRIME);
        (h % self.map.len().as_u64()).as_usize()
    }
    pub fn get(&mut self, key: &Utf8SuffixKey, hash: usize) -> Option<StateID> {
        let entry = &self.map[hash];
        if entry.version != self.version {
            return None;
        }
        if key != &entry.key {
            return None;
        }
        Some(entry.val)
    }
    pub fn set(&mut self, key: Utf8SuffixKey, hash: usize, state_id: StateID) {
        self.map[hash] = Utf8SuffixEntry {
            version: self.version,
            key,
            val: state_id,
        };
    }
}
