#[cfg(feature = "std")]
type StateMap = std::collections::HashMap<State, LazyStateID>;
#[cfg(not(feature = "std"))]
type StateMap = alloc::collections::BTreeMap<State, LazyStateID>;
use core::{iter, mem::size_of};
use alloc::vec::Vec;
use crate::{
    hybrid::{
        error::{BuildError, CacheError, StartError},
        id::{LazyStateID, LazyStateIDError},
        search,
    },
    nfa::thompson,
    util::{
        alphabet::{self, ByteClasses, ByteSet},
        determinize::{self, State, StateBuilderEmpty, StateBuilderNFA},
        empty, prefilter::Prefilter, primitives::{PatternID, StateID as NFAStateID},
        search::{Anchored, HalfMatch, Input, MatchError, MatchKind, PatternSet},
        sparse_set::SparseSets, start::{self, Start, StartByteMap},
    },
};
const MIN_STATES: usize = SENTINEL_STATES + 2;
const SENTINEL_STATES: usize = 3;
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
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
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[derive(Clone, Debug)]
pub struct DFA {
    config: Config,
    nfa: thompson::NFA,
    stride2: usize,
    start_map: StartByteMap,
    classes: ByteClasses,
    quitset: ByteSet,
    cache_capacity: usize,
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
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
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
pub struct BuildError {
    kind: BuildErrorKind,
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
impl Builder {
    pub fn new() -> Builder {}
    #[cfg(feature = "syntax")]
    pub fn build(&self, pattern: &str) -> Result<DFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError> {
        let nfa = self
            .thompson
            .clone()
            .configure(
                thompson::Config::new().which_captures(thompson::WhichCaptures::None),
            )
            .build_many(patterns)
            .map_err(BuildError::nfa)?;
        self.build_from_nfa(nfa)
    }
    pub fn build_from_nfa(&self, nfa: thompson::NFA) -> Result<DFA, BuildError> {
        let quitset = self.config.quit_set_from_nfa(&nfa)?;
        let classes = self.config.byte_classes_from_nfa(&nfa, &quitset);
        let min_cache = minimum_cache_capacity(
            &nfa,
            &classes,
            self.config.get_starts_for_each_pattern(),
        );
        let mut cache_capacity = self.config.get_cache_capacity();
        if cache_capacity < min_cache {
            if self.config.get_skip_cache_capacity_check() {
                debug!(
                    "given capacity ({}) is too small, \
                     since skip_cache_capacity_check is enabled, \
                     setting cache capacity to minimum ({})",
                    cache_capacity, min_cache,
                );
                cache_capacity = min_cache;
            } else {
                return Err(
                    BuildError::insufficient_cache_capacity(min_cache, cache_capacity),
                );
            }
        }
        if let Err(err) = minimum_lazy_state_id(&classes) {
            return Err(BuildError::insufficient_state_id_capacity(err));
        }
        let stride2 = classes.stride2();
        let start_map = StartByteMap::new(nfa.look_matcher());
        Ok(DFA {
            config: self.config.clone(),
            nfa,
            stride2,
            start_map,
            classes,
            quitset,
            cache_capacity,
        })
    }
    pub fn configure(&mut self, config: Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn thompson(&mut self, config: thompson::Config) -> &mut Builder {}
}
impl Config {
    pub fn new() -> Config {
        Config::default()
    }
    pub fn utf8(mut self, yes: bool) -> Config {}
    pub fn reverse(mut self, yes: bool) -> Config {}
    pub fn nfa_size_limit(mut self, bytes: Option<usize>) -> Config {}
    pub fn shrink(mut self, yes: bool) -> Config {}
    #[deprecated(since = "0.3.5", note = "use which_captures instead")]
    pub fn captures(self, yes: bool) -> Config {}
    pub fn which_captures(mut self, which_captures: WhichCaptures) -> Config {
        self.which_captures = Some(which_captures);
        self
    }
    pub fn look_matcher(mut self, m: LookMatcher) -> Config {}
    #[cfg(test)]
    fn unanchored_prefix(mut self, yes: bool) -> Config {}
    pub fn get_utf8(&self) -> bool {}
    pub fn get_reverse(&self) -> bool {}
    pub fn get_nfa_size_limit(&self) -> Option<usize> {}
    pub fn get_shrink(&self) -> bool {}
    #[deprecated(since = "0.3.5", note = "use get_which_captures instead")]
    pub fn get_captures(&self) -> bool {}
    pub fn get_which_captures(&self) -> WhichCaptures {}
    pub fn get_look_matcher(&self) -> LookMatcher {}
    fn get_unanchored_prefix(&self) -> bool {}
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
impl Compiler {
    pub fn new() -> Compiler {}
    pub fn build(&self, pattern: &str) -> Result<NFA, BuildError> {}
    pub fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<NFA, BuildError> {
        let mut hirs = vec![];
        for p in patterns {
            hirs.push(
                self.parser.build().parse(p.as_ref()).map_err(BuildError::syntax)?,
            );
            debug!("parsed: {:?}", p.as_ref());
        }
        self.build_many_from_hir(&hirs)
    }
    pub fn build_from_hir(&self, expr: &Hir) -> Result<NFA, BuildError> {}
    pub fn build_many_from_hir<H: Borrow<Hir>>(
        &self,
        exprs: &[H],
    ) -> Result<NFA, BuildError> {}
    pub fn configure(&mut self, config: Config) -> &mut Compiler {
        self.config = self.config.overwrite(config);
        self
    }
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Compiler {}
}
