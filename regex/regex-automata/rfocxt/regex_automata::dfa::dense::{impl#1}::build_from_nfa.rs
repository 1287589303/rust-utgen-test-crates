#[cfg(feature = "alloc")]
pub(crate) type OwnedDFA = DFA<alloc::vec::Vec<u32>>;
#[cfg(feature = "dfa-build")]
use core::cmp;
use core::{fmt, iter, mem::size_of, slice};
#[cfg(feature = "dfa-build")]
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec, vec::Vec,
};
#[cfg(feature = "dfa-build")]
use crate::{
    dfa::{accel::Accel, determinize, minimize::Minimizer, remapper::Remapper, sparse},
    nfa::thompson, util::{look::LookMatcher, search::MatchKind},
};
use crate::{
    dfa::{
        accel::Accels, automaton::{fmt_state_indicator, Automaton, StartError},
        special::Special, start::StartKind, DEAD,
    },
    util::{
        alphabet::{self, ByteClasses, ByteSet},
        int::{Pointer, Usize},
        prefilter::Prefilter, primitives::{PatternID, StateID},
        search::Anchored, start::{self, Start, StartByteMap},
        wire::{self, DeserializeError, Endian, SerializeError},
    },
};
const LABEL: &str = "rust-regex-automata-dfa-dense";
const VERSION: u32 = 2;
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
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
#[derive(Clone, Copy, Debug)]
pub(crate) struct Special {
    /// The identifier of the last special state in a DFA. A state is special
    /// if and only if its identifier is less than or equal to `max`.
    pub(crate) max: StateID,
    /// The identifier of the quit state in a DFA. (There is no analogous field
    /// for the dead state since the dead state's ID is always zero, regardless
    /// of state ID size.)
    pub(crate) quit_id: StateID,
    /// The identifier of the first match state.
    pub(crate) min_match: StateID,
    /// The identifier of the last match state.
    pub(crate) max_match: StateID,
    /// The identifier of the first accelerated state.
    pub(crate) min_accel: StateID,
    /// The identifier of the last accelerated state.
    pub(crate) max_accel: StateID,
    /// The identifier of the first start state.
    pub(crate) min_start: StateID,
    /// The identifier of the last start state.
    pub(crate) max_start: StateID,
}
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub(crate) struct ByteClassSet(ByteSet);
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Copy, Debug)]
pub(crate) struct Flags {
    /// Whether the DFA can match the empty string. When this is false, all
    /// matches returned by this DFA are guaranteed to have non-zero length.
    pub(crate) has_empty: bool,
    /// Whether the DFA should only produce matches with spans that correspond
    /// to valid UTF-8. This also includes omitting any zero-width matches that
    /// split the UTF-8 encoding of a codepoint.
    pub(crate) is_utf8: bool,
    /// Whether the DFA is always anchored or not, regardless of `Input`
    /// configuration. This is useful for avoiding a reverse scan even when
    /// executing unanchored searches.
    pub(crate) is_always_start_anchored: bool,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct LookSet {
    /// The underlying representation this set is exposed to make it possible
    /// to store it somewhere efficiently. The representation is that
    /// of a bitset, where each assertion occupies bit `i` where
    /// `i = Look::as_repr()`.
    ///
    /// Note that users of this internal representation must permit the full
    /// range of `u16` values to be represented. For example, even if the
    /// current implementation only makes use of the 10 least significant bits,
    /// it may use more bits in a future semver compatible release.
    pub bits: u32,
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
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
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
pub struct LookMatcher {
    lineterm: DebugByte,
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
#[derive(Clone, Debug)]
pub struct Prefilter {
    #[cfg(not(feature = "alloc"))]
    _unused: (),
    #[cfg(feature = "alloc")]
    pre: Arc<dyn PrefilterI>,
    #[cfg(feature = "alloc")]
    is_fast: bool,
    #[cfg(feature = "alloc")]
    max_needle_len: usize,
}
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
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
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartKind {
    /// Support both anchored and unanchored searches.
    Both,
    /// Support only unanchored searches. Requesting an anchored search will
    /// panic.
    ///
    /// Note that even if an unanchored search is requested, the pattern itself
    /// may still be anchored. For example, `^abc` will only match `abc` at the
    /// start of a haystack. This will remain true, even if the regex engine
    /// only supported unanchored searches.
    Unanchored,
    /// Support only anchored searches. Requesting an unanchored search will
    /// panic.
    Anchored,
}
#[cfg(feature = "dfa-build")]
impl Builder {
    pub fn new() -> Builder {}
    #[cfg(feature = "syntax")]
    pub fn build(&self, pattern: &str) -> Result<OwnedDFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<OwnedDFA, BuildError> {}
    pub fn build_from_nfa(&self, nfa: &thompson::NFA) -> Result<OwnedDFA, BuildError> {
        let mut quitset = self.config.quitset.unwrap_or(ByteSet::empty());
        if self.config.get_unicode_word_boundary()
            && nfa.look_set_any().contains_word_unicode()
        {
            for b in 0x80..=0xFF {
                quitset.add(b);
            }
        }
        let classes = if !self.config.get_byte_classes() {
            ByteClasses::singletons()
        } else {
            let mut set = nfa.byte_class_set().clone();
            if !quitset.is_empty() {
                set.add_set(&quitset);
            }
            set.byte_classes()
        };
        let mut dfa = DFA::initial(
            classes,
            nfa.pattern_len(),
            self.config.get_starts(),
            nfa.look_matcher(),
            self.config.get_starts_for_each_pattern(),
            self.config.get_prefilter().map(|p| p.clone()),
            quitset,
            Flags::from_nfa(&nfa),
        )?;
        determinize::Config::new()
            .match_kind(self.config.get_match_kind())
            .quit(quitset)
            .dfa_size_limit(self.config.get_dfa_size_limit())
            .determinize_size_limit(self.config.get_determinize_size_limit())
            .run(nfa, &mut dfa)?;
        if self.config.get_minimize() {
            dfa.minimize();
        }
        if self.config.get_accelerate() {
            dfa.accelerate();
        }
        if !self.config.get_specialize_start_states() {
            dfa.special.set_no_special_start_states();
        }
        dfa.set_universal_starts();
        Ok(dfa)
    }
    pub fn configure(&mut self, config: Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn thompson(&mut self, config: thompson::Config) -> &mut Builder {}
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
    pub fn pattern_len(&self) -> usize {
        self.0.start_pattern.len()
    }
    #[inline]
    pub fn start_anchored(&self) -> StateID {}
    #[inline]
    pub fn start_unanchored(&self) -> StateID {}
    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {}
    #[inline]
    pub(crate) fn byte_class_set(&self) -> &ByteClassSet {
        &self.0.byte_class_set
    }
    #[inline]
    pub fn byte_classes(&self) -> &ByteClasses {}
    #[inline]
    pub fn state(&self, id: StateID) -> &State {}
    #[inline]
    pub fn states(&self) -> &[State] {}
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
    pub fn look_matcher(&self) -> &LookMatcher {
        &self.0.look_matcher
    }
    #[inline]
    pub fn look_set_any(&self) -> LookSet {
        self.0.look_set_any
    }
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
#[cfg(feature = "dfa-build")]
impl Config {
    pub fn new() -> Config {}
    pub fn accelerate(mut self, yes: bool) -> Config {}
    pub fn prefilter(mut self, pre: Option<Prefilter>) -> Config {}
    pub fn minimize(mut self, yes: bool) -> Config {}
    pub fn match_kind(mut self, kind: MatchKind) -> Config {}
    pub fn start_kind(mut self, kind: StartKind) -> Config {}
    pub fn starts_for_each_pattern(mut self, yes: bool) -> Config {}
    pub fn byte_classes(mut self, yes: bool) -> Config {}
    pub fn unicode_word_boundary(mut self, yes: bool) -> Config {}
    pub fn quit(mut self, byte: u8, yes: bool) -> Config {}
    pub fn specialize_start_states(mut self, yes: bool) -> Config {}
    pub fn dfa_size_limit(mut self, bytes: Option<usize>) -> Config {}
    pub fn determinize_size_limit(mut self, bytes: Option<usize>) -> Config {}
    pub fn get_accelerate(&self) -> bool {
        self.accelerate.unwrap_or(true)
    }
    pub fn get_prefilter(&self) -> Option<&Prefilter> {
        self.pre.as_ref().unwrap_or(&None).as_ref()
    }
    pub fn get_minimize(&self) -> bool {
        self.minimize.unwrap_or(false)
    }
    pub fn get_match_kind(&self) -> MatchKind {
        self.match_kind.unwrap_or(MatchKind::LeftmostFirst)
    }
    pub fn get_starts(&self) -> StartKind {
        self.start_kind.unwrap_or(StartKind::Both)
    }
    pub fn get_starts_for_each_pattern(&self) -> bool {
        self.starts_for_each_pattern.unwrap_or(false)
    }
    pub fn get_byte_classes(&self) -> bool {
        self.byte_classes.unwrap_or(true)
    }
    pub fn get_unicode_word_boundary(&self) -> bool {
        self.unicode_word_boundary.unwrap_or(false)
    }
    pub fn get_quit(&self, byte: u8) -> bool {}
    pub fn get_specialize_start_states(&self) -> bool {
        self.specialize_start_states.unwrap_or(false)
    }
    pub fn get_dfa_size_limit(&self) -> Option<usize> {
        self.dfa_size_limit.unwrap_or(None)
    }
    pub fn get_determinize_size_limit(&self) -> Option<usize> {
        self.determinize_size_limit.unwrap_or(None)
    }
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
impl Config {
    pub fn new() -> Config {
        Config {
            match_kind: MatchKind::LeftmostFirst,
            quit: ByteSet::empty(),
            dfa_size_limit: None,
            determinize_size_limit: None,
        }
    }
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
    pub fn match_kind(&mut self, kind: MatchKind) -> &mut Config {
        self.match_kind = kind;
        self
    }
    pub fn quit(&mut self, set: ByteSet) -> &mut Config {
        self.quit = set;
        self
    }
    pub fn dfa_size_limit(&mut self, bytes: Option<usize>) -> &mut Config {
        self.dfa_size_limit = bytes;
        self
    }
    pub fn determinize_size_limit(&mut self, bytes: Option<usize>) -> &mut Config {
        self.determinize_size_limit = bytes;
        self
    }
}
impl Special {
    #[cfg(feature = "dfa-build")]
    pub(crate) fn new() -> Special {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn remap(&self, map: impl Fn(StateID) -> StateID) -> Special {}
    pub(crate) fn from_bytes(
        mut slice: &[u8],
    ) -> Result<(Special, usize), DeserializeError> {}
    pub(crate) fn validate(&self) -> Result<(), DeserializeError> {}
    pub(crate) fn validate_state_len(
        &self,
        len: usize,
        stride2: usize,
    ) -> Result<(), DeserializeError> {}
    pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn set_max(&mut self) {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn set_no_special_start_states(&mut self) {
        use core::cmp::max;
        self.max = max(self.quit_id, max(self.max_match, self.max_accel));
        self.min_start = DEAD;
        self.max_start = DEAD;
    }
    #[inline]
    pub(crate) fn is_special_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_dead_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_quit_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_match_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_accel_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_start_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn match_len(&self, stride: usize) -> usize {}
    #[inline]
    pub(crate) fn matches(&self) -> bool {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn accel_len(&self, stride: usize) -> usize {}
    #[inline]
    pub(crate) fn accels(&self) -> bool {}
    #[inline]
    pub(crate) fn starts(&self) -> bool {}
}
#[cfg(feature = "alloc")]
impl ByteClassSet {
    pub(crate) fn empty() -> Self {
        ByteClassSet(ByteSet::empty())
    }
    pub(crate) fn set_range(&mut self, start: u8, end: u8) {}
    pub(crate) fn add_set(&mut self, set: &ByteSet) {
        for (start, end) in set.iter_ranges() {
            self.set_range(start, end);
        }
    }
    pub(crate) fn byte_classes(&self) -> ByteClasses {
        let mut classes = ByteClasses::empty();
        let mut class = 0u8;
        let mut b = 0u8;
        loop {
            classes.set(b, class);
            if b == 255 {
                break;
            }
            if self.0.contains(b) {
                class = class.checked_add(1).unwrap();
            }
            b = b.checked_add(1).unwrap();
        }
        classes
    }
}
impl ByteClasses {
    #[inline]
    pub fn empty() -> ByteClasses {}
    #[inline]
    pub fn singletons() -> ByteClasses {
        let mut classes = ByteClasses::empty();
        for b in 0..=255 {
            classes.set(b, b);
        }
        classes
    }
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteClasses, usize), DeserializeError> {}
    pub(crate) fn write_to(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
    #[inline]
    pub fn set(&mut self, byte: u8, class: u8) {}
    #[inline]
    pub fn get(&self, byte: u8) -> u8 {}
    #[inline]
    pub fn get_by_unit(&self, unit: Unit) -> usize {}
    #[inline]
    pub fn eoi(&self) -> Unit {}
    #[inline]
    pub fn alphabet_len(&self) -> usize {}
    #[inline]
    pub fn stride2(&self) -> usize {}
    #[inline]
    pub fn is_singleton(&self) -> bool {}
    #[inline]
    pub fn iter(&self) -> ByteClassIter<'_> {}
    pub fn representatives<R: core::ops::RangeBounds<u8>>(
        &self,
        range: R,
    ) -> ByteClassRepresentatives<'_> {}
    #[inline]
    pub fn elements(&self, class: Unit) -> ByteClassElements {}
    fn element_ranges(&self, class: Unit) -> ByteClassElementRanges {}
}
impl Flags {
    #[cfg(feature = "dfa-build")]
    fn from_nfa(nfa: &thompson::NFA) -> Flags {
        Flags {
            has_empty: nfa.has_empty(),
            is_utf8: nfa.is_utf8(),
            is_always_start_anchored: nfa.is_always_start_anchored(),
        }
    }
    pub(crate) fn from_bytes(slice: &[u8]) -> Result<(Flags, usize), DeserializeError> {}
    pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
}
impl ByteSet {
    pub(crate) fn empty() -> ByteSet {
        ByteSet { bits: BitSet([0; 2]) }
    }
    pub(crate) fn add(&mut self, byte: u8) {
        let bucket = byte / 128;
        let bit = byte % 128;
        self.bits.0[usize::from(bucket)] |= 1 << bit;
    }
    pub(crate) fn remove(&mut self, byte: u8) {}
    pub(crate) fn contains(&self, byte: u8) -> bool {}
    pub(crate) fn contains_range(&self, start: u8, end: u8) -> bool {}
    pub(crate) fn iter(&self) -> ByteSetIter {}
    pub(crate) fn iter_ranges(&self) -> ByteSetRangeIter {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_empty(&self) -> bool {
        self.bits.0 == [0, 0]
    }
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteSet, usize), DeserializeError> {}
    pub(crate) fn write_to<E: crate::util::wire::Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
}
impl LookSet {
    #[inline]
    pub fn empty() -> LookSet {}
    #[inline]
    pub fn full() -> LookSet {}
    #[inline]
    pub fn singleton(look: Look) -> LookSet {}
    #[inline]
    pub fn len(self) -> usize {}
    #[inline]
    pub fn is_empty(self) -> bool {}
    #[inline]
    pub fn contains(self, look: Look) -> bool {}
    #[inline]
    pub fn contains_anchor(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_haystack(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_line(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_lf(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_crlf(&self) -> bool {}
    #[inline]
    pub fn contains_word(self) -> bool {}
    #[inline]
    pub fn contains_word_unicode(self) -> bool {
        self.contains(Look::WordUnicode) || self.contains(Look::WordUnicodeNegate)
            || self.contains(Look::WordStartUnicode)
            || self.contains(Look::WordEndUnicode)
            || self.contains(Look::WordStartHalfUnicode)
            || self.contains(Look::WordEndHalfUnicode)
    }
    #[inline]
    pub fn contains_word_ascii(self) -> bool {}
    #[inline]
    pub fn iter(self) -> LookSetIter {}
    #[inline]
    pub fn insert(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_insert(&mut self, look: Look) {}
    #[inline]
    pub fn remove(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_remove(&mut self, look: Look) {}
    #[inline]
    pub fn subtract(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_subtract(&mut self, other: LookSet) {}
    #[inline]
    pub fn union(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_union(&mut self, other: LookSet) {}
    #[inline]
    pub fn intersect(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_intersect(&mut self, other: LookSet) {}
    #[inline]
    pub fn read_repr(slice: &[u8]) -> LookSet {}
    #[inline]
    pub fn write_repr(self, slice: &mut [u8]) {}
    pub fn available(self) -> Result<(), UnicodeWordBoundaryError> {}
}
