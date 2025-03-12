use core::{fmt::Debug, panic::{RefUnwindSafe, UnwindSafe}};
use alloc::sync::Arc;
use regex_syntax::hir::{literal, Hir};
use crate::{
    meta::{
        error::{BuildError, RetryError, RetryFailError, RetryQuadraticError},
        regex::{Cache, RegexInfo},
        reverse_inner, wrappers,
    },
    nfa::thompson::{self, WhichCaptures, NFA},
    util::{
        captures::{Captures, GroupInfo},
        look::LookMatcher, prefilter::{self, Prefilter, PrefilterI},
        primitives::{NonMaxUsize, PatternID},
        search::{Anchored, HalfMatch, Input, Match, MatchKind, PatternSet},
    },
};
pub(super) trait Strategy: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn group_info(&self) -> &GroupInfo;
    fn create_cache(&self) -> Cache;
    fn reset_cache(&self, cache: &mut Cache);
    fn is_accelerated(&self) -> bool;
    fn memory_usage(&self) -> usize;
    fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>;
    fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>;
    fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool;
    fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID>;
    fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    );
}
#[derive(Debug)]
struct ReverseInner {
    core: Core,
    preinner: Prefilter,
    nfarev: NFA,
    hybrid: wrappers::ReverseHybrid,
    dfa: wrappers::ReverseDFA,
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
#[derive(Debug)]
pub(crate) struct DFA(Option<DFAEngine>);
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
#[derive(Debug)]
pub(crate) struct ReverseHybrid(Option<ReverseHybridEngine>);
#[derive(Clone, Debug)]
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
#[derive(Clone, Debug)]
pub struct LookMatcher {
    lineterm: DebugByte,
}
#[derive(Debug)]
pub(crate) struct ReverseDFA(Option<ReverseDFAEngine>);
#[derive(Debug)]
pub(crate) struct Hybrid(Option<HybridEngine>);
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
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Debug)]
struct Core {
    info: RegexInfo,
    pre: Option<Prefilter>,
    nfa: NFA,
    nfarev: Option<NFA>,
    pikevm: wrappers::PikeVM,
    backtrack: wrappers::BoundedBacktracker,
    onepass: wrappers::OnePass,
    hybrid: wrappers::Hybrid,
    dfa: wrappers::DFA,
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
impl ReverseInner {
    fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseInner, Core> {
        if !core.info.config().get_auto_prefilter() {
            debug!(
                "skipping reverse inner optimization because \
                 automatic prefilters are disabled"
            );
            return Err(core);
        }
        if core.info.config().get_match_kind() != MatchKind::LeftmostFirst {
            debug!(
                "skipping reverse inner optimization because \
				 match kind is {:?} but this only supports leftmost-first",
                core.info.config().get_match_kind(),
            );
            return Err(core);
        }
        if core.info.is_always_anchored_start() {
            debug!(
                "skipping reverse inner optimization because \
				 the regex is always anchored at the start",
            );
            return Err(core);
        }
        if !core.hybrid.is_some() && !core.dfa.is_some() {
            debug!(
                "skipping reverse inner optimization because \
				 we don't have a lazy DFA or a full DFA"
            );
            return Err(core);
        }
        if core.pre.as_ref().map_or(false, |p| p.is_fast()) {
            debug!(
                "skipping reverse inner optimization because \
				 we already have a prefilter that we think is fast"
            );
            return Err(core);
        } else if core.pre.is_some() {
            debug!(
                "core engine has a prefix prefilter, but it is \
                 probably not fast, so continuing with attempt to \
                 use reverse inner prefilter"
            );
        }
        let (concat_prefix, preinner) = match reverse_inner::extract(hirs) {
            Some(x) => x,
            None => return Err(core),
        };
        debug!("building reverse NFA for prefix before inner literal");
        let mut lookm = LookMatcher::new();
        lookm.set_line_terminator(core.info.config().get_line_terminator());
        let thompson_config = thompson::Config::new()
            .reverse(true)
            .utf8(core.info.config().get_utf8_empty())
            .nfa_size_limit(core.info.config().get_nfa_size_limit())
            .shrink(false)
            .which_captures(WhichCaptures::None)
            .look_matcher(lookm);
        let result = thompson::Compiler::new()
            .configure(thompson_config)
            .build_from_hir(&concat_prefix);
        let nfarev = match result {
            Ok(nfarev) => nfarev,
            Err(_err) => {
                debug!(
                    "skipping reverse inner optimization because the \
					 reverse NFA failed to build: {}",
                    _err,
                );
                return Err(core);
            }
        };
        debug!("building reverse DFA for prefix before inner literal");
        let dfa = if !core.info.config().get_dfa() {
            wrappers::ReverseDFA::none()
        } else {
            wrappers::ReverseDFA::new(&core.info, &nfarev)
        };
        let hybrid = if !core.info.config().get_hybrid() {
            wrappers::ReverseHybrid::none()
        } else if dfa.is_some() {
            debug!(
                "skipping lazy DFA for reverse inner optimization \
				 because we have a full DFA"
            );
            wrappers::ReverseHybrid::none()
        } else {
            wrappers::ReverseHybrid::new(&core.info, &nfarev)
        };
        Ok(ReverseInner {
            core,
            preinner,
            nfarev,
            hybrid,
            dfa,
        })
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_full(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Option<Match>, RetryError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_half_fwd_stopat(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Result<HalfMatch, usize>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_half_rev_limited(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {}
}
impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            parser: ParserBuilder::new(),
            config: Config::default(),
            builder: RefCell::new(Builder::new()),
            utf8_state: RefCell::new(Utf8State::new()),
            trie_state: RefCell::new(RangeTrie::new()),
            utf8_suffix: RefCell::new(Utf8SuffixMap::new(1000)),
        }
    }
    pub fn build(&self, pattern: &str) -> Result<NFA, BuildError> {}
    pub fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<NFA, BuildError> {}
    pub fn build_from_hir(&self, expr: &Hir) -> Result<NFA, BuildError> {
        self.build_many_from_hir(&[expr])
    }
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
impl Config {
    pub fn new() -> Config {
        Config::default()
    }
    pub fn match_kind(self, kind: MatchKind) -> Config {}
    pub fn utf8_empty(self, yes: bool) -> Config {}
    pub fn auto_prefilter(self, yes: bool) -> Config {}
    pub fn prefilter(self, pre: Option<Prefilter>) -> Config {}
    pub fn which_captures(mut self, which_captures: WhichCaptures) -> Config {
        self.which_captures = Some(which_captures);
        self
    }
    pub fn nfa_size_limit(mut self, bytes: Option<usize>) -> Config {
        self.nfa_size_limit = Some(bytes);
        self
    }
    pub fn onepass_size_limit(self, limit: Option<usize>) -> Config {}
    pub fn hybrid_cache_capacity(self, limit: usize) -> Config {}
    pub fn dfa_size_limit(self, limit: Option<usize>) -> Config {}
    pub fn dfa_state_limit(self, limit: Option<usize>) -> Config {}
    pub fn byte_classes(self, yes: bool) -> Config {}
    pub fn line_terminator(self, byte: u8) -> Config {}
    pub fn hybrid(self, yes: bool) -> Config {}
    pub fn dfa(self, yes: bool) -> Config {}
    pub fn onepass(self, yes: bool) -> Config {}
    pub fn backtrack(self, yes: bool) -> Config {}
    pub fn get_match_kind(&self) -> MatchKind {
        self.match_kind.unwrap_or(MatchKind::LeftmostFirst)
    }
    pub fn get_utf8_empty(&self) -> bool {
        self.utf8_empty.unwrap_or(true)
    }
    pub fn get_auto_prefilter(&self) -> bool {
        self.autopre.unwrap_or(true)
    }
    pub fn get_prefilter(&self) -> Option<&Prefilter> {}
    pub fn get_which_captures(&self) -> WhichCaptures {}
    pub fn get_nfa_size_limit(&self) -> Option<usize> {
        self.nfa_size_limit.unwrap_or(Some(10 * (1 << 20)))
    }
    pub fn get_onepass_size_limit(&self) -> Option<usize> {}
    pub fn get_hybrid_cache_capacity(&self) -> usize {}
    pub fn get_dfa_size_limit(&self) -> Option<usize> {}
    pub fn get_dfa_state_limit(&self) -> Option<usize> {}
    pub fn get_byte_classes(&self) -> bool {}
    pub fn get_line_terminator(&self) -> u8 {
        self.line_terminator.unwrap_or(b'\n')
    }
    pub fn get_hybrid(&self) -> bool {
        #[cfg(feature = "hybrid")] { self.hybrid.unwrap_or(true) }
        #[cfg(not(feature = "hybrid"))] { false }
    }
    pub fn get_dfa(&self) -> bool {
        #[cfg(feature = "dfa-build")] { self.dfa.unwrap_or(true) }
        #[cfg(not(feature = "dfa-build"))] { false }
    }
    pub fn get_onepass(&self) -> bool {}
    pub fn get_backtrack(&self) -> bool {}
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
impl DFA {
    pub(crate) fn none() -> DFA {}
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> DFA {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&DFAEngine> {}
    pub(crate) fn is_some(&self) -> bool {
        self.0.is_some()
    }
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl ReverseHybrid {
    pub(crate) fn none() -> ReverseHybrid {
        ReverseHybrid(None)
    }
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseHybrid {
        ReverseHybrid(ReverseHybridEngine::new(info, nfarev))
    }
    pub(crate) fn create_cache(&self) -> ReverseHybridCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseHybridEngine> {}
}
impl RegexInfo {
    fn new(config: Config, hirs: &[&Hir]) -> RegexInfo {}
    pub(crate) fn config(&self) -> &Config {
        &self.0.config
    }
    pub(crate) fn props(&self) -> &[hir::Properties] {}
    pub(crate) fn props_union(&self) -> &hir::Properties {}
    pub(crate) fn pattern_len(&self) -> usize {}
    pub(crate) fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_anchored_start(&self, input: &Input<'_>) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_always_anchored_start(&self) -> bool {
        use regex_syntax::hir::Look;
        self.props_union().look_set_prefix().contains(Look::Start)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_always_anchored_end(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_impossible(&self, input: &Input<'_>) -> bool {}
}
impl LookMatcher {
    pub fn new() -> LookMatcher {
        LookMatcher {
            lineterm: DebugByte(b'\n'),
        }
    }
    pub fn set_line_terminator(&mut self, byte: u8) -> &mut LookMatcher {
        self.lineterm.0 = byte;
        self
    }
    pub fn get_line_terminator(&self) -> u8 {}
    #[inline]
    pub fn matches(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_inline(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn matches_set(&self, set: LookSet, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_set_inline(
        &self,
        set: LookSet,
        haystack: &[u8],
        at: usize,
    ) -> bool {}
    #[cfg(feature = "alloc")]
    pub(crate) fn add_to_byteset(
        &self,
        look: Look,
        set: &mut crate::util::alphabet::ByteClassSet,
    ) {}
    #[inline]
    pub fn is_start(&self, _haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii_negate(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_unicode_negate(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
}
impl ReverseDFA {
    pub(crate) fn none() -> ReverseDFA {
        ReverseDFA(None)
    }
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseDFA {
        ReverseDFA(ReverseDFAEngine::new(info, nfarev))
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseDFAEngine> {}
    pub(crate) fn is_some(&self) -> bool {
        self.0.is_some()
    }
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl Hybrid {
    pub(crate) fn none() -> Hybrid {}
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Hybrid {}
    pub(crate) fn create_cache(&self) -> HybridCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&HybridEngine> {}
    pub(crate) fn is_some(&self) -> bool {
        self.0.is_some()
    }
}
pub(crate) fn extract(hirs: &[&Hir]) -> Option<(Hir, Prefilter)> {
    if hirs.len() != 1 {
        debug!(
            "skipping reverse inner optimization since it only \
		 	 supports 1 pattern, {} were given",
            hirs.len(),
        );
        return None;
    }
    let mut concat = match top_concat(hirs[0]) {
        Some(concat) => concat,
        None => {
            debug!(
                "skipping reverse inner optimization because a top-level \
		 	     concatenation could not found",
            );
            return None;
        }
    };
    for i in 1..concat.len() {
        let hir = &concat[i];
        let pre = match prefilter(hir) {
            None => continue,
            Some(pre) => pre,
        };
        if !pre.is_fast() {
            debug!(
                "skipping extracted inner prefilter because \
				 it probably isn't fast"
            );
            continue;
        }
        let concat_suffix = Hir::concat(concat.split_off(i));
        let concat_prefix = Hir::concat(concat);
        let pre2 = match prefilter(&concat_suffix) {
            None => pre,
            Some(pre2) => if pre2.is_fast() { pre2 } else { pre }
        };
        return Some((concat_prefix, pre2));
    }
    debug!(
        "skipping reverse inner optimization because a top-level \
	     sub-expression with a fast prefilter could not be found"
    );
    None
}
