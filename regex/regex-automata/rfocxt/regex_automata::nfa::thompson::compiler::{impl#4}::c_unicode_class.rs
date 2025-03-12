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
#[derive(Debug)]
struct Utf8Compiler<'a> {
    builder: &'a mut Builder,
    state: &'a mut Utf8State,
    target: StateID,
}
#[derive(Clone, Debug)]
pub struct Builder {
    #[cfg(feature = "dfa-build")]
    dfa: dense::Builder,
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
struct Utf8State {
    compiled: Utf8BoundedMap,
    uncompiled: Vec<Utf8Node>,
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
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
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
#[derive(Clone, Copy, Debug)]
pub(crate) struct ThompsonRef {
    pub(crate) start: StateID,
    pub(crate) end: StateID,
}
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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
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
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
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
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
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
    ) -> Result<ThompsonRef, BuildError> {
        if cls.is_ascii() {
            let end = self.add_empty()?;
            let mut trans = Vec::with_capacity(cls.ranges().len());
            for r in cls.iter() {
                trans
                    .push(Transition {
                        start: u8::try_from(u32::from(r.start())).unwrap(),
                        end: u8::try_from(u32::from(r.end())).unwrap(),
                        next: end,
                    });
            }
            Ok(ThompsonRef {
                start: self.add_sparse(trans)?,
                end,
            })
        } else if self.is_reverse() {
            if !self.config.get_shrink() {
                self.c_unicode_class_reverse_with_suffix(cls)
            } else {
                let mut trie = self.trie_state.borrow_mut();
                trie.clear();
                for rng in cls.iter() {
                    for mut seq in Utf8Sequences::new(rng.start(), rng.end()) {
                        seq.reverse();
                        trie.insert(seq.as_slice());
                    }
                }
                let mut builder = self.builder.borrow_mut();
                let mut utf8_state = self.utf8_state.borrow_mut();
                let mut utf8c = Utf8Compiler::new(&mut *builder, &mut *utf8_state)?;
                trie.iter(|seq| {
                    utf8c.add(&seq)?;
                    Ok(())
                })?;
                utf8c.finish()
            }
        } else {
            let mut builder = self.builder.borrow_mut();
            let mut utf8_state = self.utf8_state.borrow_mut();
            let mut utf8c = Utf8Compiler::new(&mut *builder, &mut *utf8_state)?;
            for rng in cls.iter() {
                for seq in Utf8Sequences::new(rng.start(), rng.end()) {
                    utf8c.add(seq.as_slice())?;
                }
            }
            utf8c.finish()
        }
    }
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
    fn c_range(&self, start: u8, end: u8) -> Result<ThompsonRef, BuildError> {}
    fn c_empty(&self) -> Result<ThompsonRef, BuildError> {}
    fn c_fail(&self) -> Result<ThompsonRef, BuildError> {}
    fn patch(&self, from: StateID, to: StateID) -> Result<(), BuildError> {}
    fn start_pattern(&self) -> Result<PatternID, BuildError> {}
    fn finish_pattern(&self, start_id: StateID) -> Result<PatternID, BuildError> {}
    fn add_empty(&self) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_empty()
    }
    fn add_range(&self, start: u8, end: u8) -> Result<StateID, BuildError> {}
    fn add_sparse(&self, ranges: Vec<Transition>) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_sparse(ranges)
    }
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
    fn is_reverse(&self) -> bool {
        self.config.get_reverse()
    }
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
    pub fn get_utf8(&self) -> bool {}
    pub fn get_reverse(&self) -> bool {}
    pub fn get_nfa_size_limit(&self) -> Option<usize> {}
    pub fn get_shrink(&self) -> bool {
        self.shrink.unwrap_or(false)
    }
    #[deprecated(since = "0.3.5", note = "use get_which_captures instead")]
    pub fn get_captures(&self) -> bool {}
    pub fn get_which_captures(&self) -> WhichCaptures {}
    pub fn get_look_matcher(&self) -> LookMatcher {}
    fn get_unanchored_prefix(&self) -> bool {}
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
impl RangeTrie {
    pub fn new() -> RangeTrie {}
    pub fn clear(&mut self) {
        self.free.extend(self.states.drain(..));
        self.add_empty();
        self.add_empty();
    }
    pub fn iter<E, F: FnMut(&[Utf8Range]) -> Result<(), E>>(
        &self,
        mut f: F,
    ) -> Result<(), E> {
        let mut stack = self.iter_stack.borrow_mut();
        stack.clear();
        let mut ranges = self.iter_ranges.borrow_mut();
        ranges.clear();
        stack
            .push(NextIter {
                state_id: ROOT,
                tidx: 0,
            });
        while let Some(NextIter { mut state_id, mut tidx }) = stack.pop() {
            loop {
                let state = self.state(state_id);
                if tidx >= state.transitions.len() {
                    ranges.pop();
                    break;
                }
                let t = &state.transitions[tidx];
                ranges.push(t.range);
                if t.next_id == FINAL {
                    f(&ranges)?;
                    ranges.pop();
                    tidx += 1;
                } else {
                    stack
                        .push(NextIter {
                            state_id,
                            tidx: tidx + 1,
                        });
                    state_id = t.next_id;
                    tidx = 0;
                }
            }
        }
        Ok(())
    }
    pub fn insert(&mut self, ranges: &[Utf8Range]) {
        assert!(! ranges.is_empty());
        assert!(ranges.len() <= 4);
        let mut stack = mem::replace(&mut self.insert_stack, vec![]);
        stack.clear();
        stack.push(NextInsert::new(ROOT, ranges));
        while let Some(next) = stack.pop() {
            let (state_id, ranges) = (next.state_id(), next.ranges());
            assert!(! ranges.is_empty());
            let (mut new, rest) = (ranges[0], &ranges[1..]);
            let mut i = self.state(state_id).find(new);
            if i == self.state(state_id).transitions.len() {
                let next_id = NextInsert::push(self, &mut stack, rest);
                self.add_transition(state_id, new, next_id);
                continue;
            }
            'OUTER: loop {
                let old = self.state(state_id).transitions[i].clone();
                let split = match Split::new(old.range, new) {
                    Some(split) => split,
                    None => {
                        let next_id = NextInsert::push(self, &mut stack, rest);
                        self.add_transition_at(i, state_id, new, next_id);
                        continue;
                    }
                };
                let splits = split.as_slice();
                if splits.len() == 1 {
                    if !rest.is_empty() {
                        stack.push(NextInsert::new(old.next_id, rest));
                    }
                    break;
                }
                let mut first = true;
                let mut add_trans = |trie: &mut RangeTrie, pos, from, range, to| {
                    if first {
                        trie.set_transition_at(pos, from, range, to);
                        first = false;
                    } else {
                        trie.add_transition_at(pos, from, range, to);
                    }
                };
                for (j, &srange) in splits.iter().enumerate() {
                    match srange {
                        SplitRange::Old(r) => {
                            let dup_id = self.duplicate(old.next_id);
                            add_trans(self, i, state_id, r, dup_id);
                        }
                        SplitRange::New(r) => {
                            {
                                let trans = &self.state(state_id).transitions;
                                if j + 1 == splits.len() && i < trans.len()
                                    && intersects(r, trans[i].range)
                                {
                                    new = r;
                                    continue 'OUTER;
                                }
                            }
                            let next_id = NextInsert::push(self, &mut stack, rest);
                            add_trans(self, i, state_id, r, next_id);
                        }
                        SplitRange::Both(r) => {
                            if !rest.is_empty() {
                                stack.push(NextInsert::new(old.next_id, rest));
                            }
                            add_trans(self, i, state_id, r, old.next_id);
                        }
                    }
                    i += 1;
                }
                break;
            }
        }
        self.insert_stack = stack;
    }
    pub fn add_empty(&mut self) -> StateID {}
    fn duplicate(&mut self, old_id: StateID) -> StateID {}
    fn add_transition(&mut self, from_id: StateID, range: Utf8Range, next_id: StateID) {}
    fn add_transition_at(
        &mut self,
        i: usize,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {}
    fn set_transition_at(
        &mut self,
        i: usize,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {}
    fn state(&self, id: StateID) -> &State {}
    fn state_mut(&mut self, id: StateID) -> &mut State {}
}
impl<'a> Utf8Compiler<'a> {
    fn new(
        builder: &'a mut Builder,
        state: &'a mut Utf8State,
    ) -> Result<Utf8Compiler<'a>, BuildError> {
        let target = builder.add_empty()?;
        state.clear();
        let mut utf8c = Utf8Compiler {
            builder,
            state,
            target,
        };
        utf8c.add_empty();
        Ok(utf8c)
    }
    fn finish(&mut self) -> Result<ThompsonRef, BuildError> {
        self.compile_from(0)?;
        let node = self.pop_root();
        let start = self.compile(node)?;
        Ok(ThompsonRef {
            start,
            end: self.target,
        })
    }
    fn add(&mut self, ranges: &[Utf8Range]) -> Result<(), BuildError> {
        let prefix_len = ranges
            .iter()
            .zip(&self.state.uncompiled)
            .take_while(|&(range, node)| {
                node.last
                    .as_ref()
                    .map_or(false, |t| { (t.start, t.end) == (range.start, range.end) })
            })
            .count();
        assert!(prefix_len < ranges.len());
        self.compile_from(prefix_len)?;
        self.add_suffix(&ranges[prefix_len..]);
        Ok(())
    }
    fn compile_from(&mut self, from: usize) -> Result<(), BuildError> {}
    fn compile(&mut self, node: Vec<Transition>) -> Result<StateID, BuildError> {}
    fn add_suffix(&mut self, ranges: &[Utf8Range]) {}
    fn add_empty(&mut self) {}
    fn pop_freeze(&mut self, next: StateID) -> Vec<Transition> {}
    fn pop_root(&mut self) -> Vec<Transition> {}
    fn top_last_freeze(&mut self, next: StateID) {}
}
