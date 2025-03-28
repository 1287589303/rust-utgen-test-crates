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
#[derive(Debug)]
struct Utf8Compiler<'a> {
    builder: &'a mut Builder,
    state: &'a mut Utf8State,
    target: StateID,
}
#[derive(Clone, Debug)]
pub struct Utf8BoundedMap {
    /// The current version of this map. Only entries with matching versions
    /// are considered during lookups. If an entry is found with a mismatched
    /// version, then the map behaves as if the entry does not exist.
    ///
    /// This makes it possible to clear the map by simply incrementing the
    /// version number instead of actually deallocating any storage.
    version: u16,
    /// The total number of entries this map can store.
    capacity: usize,
    /// The actual entries, keyed by hash. Collisions between different states
    /// result in the old state being dropped.
    map: Vec<Utf8BoundedEntry>,
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
#[derive(Clone, Debug)]
struct Utf8State {
    compiled: Utf8BoundedMap,
    uncompiled: Vec<Utf8Node>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
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
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Debug)]
pub struct Builder {
    dfa: dfa::Builder,
}
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
impl<'a> Utf8Compiler<'a> {
    fn new(
        builder: &'a mut Builder,
        state: &'a mut Utf8State,
    ) -> Result<Utf8Compiler<'a>, BuildError> {}
    fn finish(&mut self) -> Result<ThompsonRef, BuildError> {}
    fn add(&mut self, ranges: &[Utf8Range]) -> Result<(), BuildError> {}
    fn compile_from(&mut self, from: usize) -> Result<(), BuildError> {}
    fn compile(&mut self, node: Vec<Transition>) -> Result<StateID, BuildError> {
        let hash = self.state.compiled.hash(&node);
        if let Some(id) = self.state.compiled.get(&node, hash) {
            return Ok(id);
        }
        let id = self.builder.add_sparse(node.clone())?;
        self.state.compiled.set(node, hash, id);
        Ok(id)
    }
    fn add_suffix(&mut self, ranges: &[Utf8Range]) {}
    fn add_empty(&mut self) {}
    fn pop_freeze(&mut self, next: StateID) -> Vec<Transition> {}
    fn pop_root(&mut self) -> Vec<Transition> {}
    fn top_last_freeze(&mut self, next: StateID) {}
}
impl Utf8BoundedMap {
    pub fn new(capacity: usize) -> Utf8BoundedMap {}
    pub fn clear(&mut self) {}
    pub fn hash(&self, key: &[Transition]) -> usize {
        let mut h = INIT;
        for t in key {
            h = (h ^ u64::from(t.start)).wrapping_mul(PRIME);
            h = (h ^ u64::from(t.end)).wrapping_mul(PRIME);
            h = (h ^ t.next.as_u64()).wrapping_mul(PRIME);
        }
        (h % self.map.len().as_u64()).as_usize()
    }
    pub fn get(&mut self, key: &[Transition], hash: usize) -> Option<StateID> {
        let entry = &self.map[hash];
        if entry.version != self.version {
            return None;
        }
        if entry.key != key {
            return None;
        }
        Some(entry.val)
    }
    pub fn set(&mut self, key: Vec<Transition>, hash: usize, state_id: StateID) {
        self.map[hash] = Utf8BoundedEntry {
            version: self.version,
            key,
            val: state_id,
        };
    }
}
impl Builder {
    pub fn new() -> Builder {}
    pub fn clear(&mut self) {}
    pub fn build(
        &self,
        start_anchored: StateID,
        start_unanchored: StateID,
    ) -> Result<NFA, BuildError> {}
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
    ) -> Result<StateID, BuildError> {
        self.add(State::Sparse { transitions })
    }
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
    pub fn set_utf8(&mut self, yes: bool) {}
    pub fn get_utf8(&self) -> bool {}
    pub fn set_reverse(&mut self, yes: bool) {}
    pub fn get_reverse(&self) -> bool {}
    pub fn set_look_matcher(&mut self, m: LookMatcher) {}
    pub fn get_look_matcher(&self) -> &LookMatcher {}
    pub fn set_size_limit(&mut self, limit: Option<usize>) -> Result<(), BuildError> {}
    pub fn get_size_limit(&self) -> Option<usize> {}
    pub fn memory_usage(&self) -> usize {}
    fn check_size_limit(&self) -> Result<(), BuildError> {}
}
