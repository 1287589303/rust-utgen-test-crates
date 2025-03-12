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
#[derive(Clone, Debug)]
struct Pre<P> {
    pre: P,
    group_info: GroupInfo,
}
#[derive(Clone, Debug)]
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
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
pub(crate) struct Memmem {
    #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
    _unused: (),
    #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
    finder: memchr::memmem::Finder<'static>,
}
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[derive(Clone, Debug)]
pub(crate) struct Memchr2(u8, u8);
#[derive(Clone, Debug)]
pub(crate) struct Teddy {
    #[cfg(not(feature = "perf-literal-multisubstring"))]
    _unused: (),
    /// The actual Teddy searcher.
    ///
    /// Technically, it's possible that Teddy doesn't actually get used, since
    /// Teddy does require its haystack to at least be of a certain size
    /// (usually around the size of whatever vector is being used, so ~16
    /// or ~32 bytes). For haystacks shorter than that, the implementation
    /// currently uses Rabin-Karp.
    #[cfg(feature = "perf-literal-multisubstring")]
    searcher: aho_corasick::packed::Searcher,
    /// When running an anchored search, the packed searcher can't handle it so
    /// we defer to Aho-Corasick itself. Kind of sad, but changing the packed
    /// searchers to support anchored search would be difficult at worst and
    /// annoying at best. Since packed searchers only apply to small numbers of
    /// literals, we content ourselves that this is not much of an added cost.
    /// (That packed searchers only work with a small number of literals is
    /// also why we use a DFA here. Otherwise, the memory usage of a DFA would
    /// likely be unacceptable.)
    #[cfg(feature = "perf-literal-multisubstring")]
    anchored_ac: aho_corasick::dfa::DFA,
    /// The length of the smallest literal we look for.
    ///
    /// We use this as a heuristic to figure out whether this will be "fast" or
    /// not. Generally, the longer the better, because longer needles are more
    /// discriminating and thus reduce false positive rate.
    #[cfg(feature = "perf-literal-multisubstring")]
    minimum_len: usize,
}
#[derive(Clone, Debug)]
pub(crate) struct Memchr(u8);
#[derive(Clone, Debug)]
pub(crate) struct Memchr3(u8, u8, u8);
#[derive(Clone, Debug)]
pub(crate) struct AhoCorasick {
    #[cfg(not(feature = "perf-literal-multisubstring"))]
    _unused: (),
    #[cfg(feature = "perf-literal-multisubstring")]
    ac: aho_corasick::AhoCorasick,
}
#[derive(Clone, Debug)]
pub(crate) enum Choice {
    Memchr(Memchr),
    Memchr2(Memchr2),
    Memchr3(Memchr3),
    Memmem(Memmem),
    Teddy(Teddy),
    ByteSet(ByteSet),
    AhoCorasick(AhoCorasick),
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
impl Pre<()> {
    fn from_prefixes(
        info: &RegexInfo,
        prefixes: &literal::Seq,
    ) -> Option<Arc<dyn Strategy>> {
        let kind = info.config().get_match_kind();
        if !prefixes.is_exact() {
            return None;
        }
        if info.pattern_len() != 1 {
            return None;
        }
        if info.props()[0].explicit_captures_len() != 0 {
            return None;
        }
        if !info.props()[0].look_set().is_empty() {
            return None;
        }
        if kind != MatchKind::LeftmostFirst {
            return None;
        }
        let prefixes = prefixes.literals().unwrap();
        debug!(
            "trying to bypass regex engine by creating \
             prefilter from {} literals: {:?}",
            prefixes.len(), prefixes,
        );
        let choice = match prefilter::Choice::new(kind, prefixes) {
            Some(choice) => choice,
            None => {
                debug!("regex bypass failed because no prefilter could be built");
                return None;
            }
        };
        let strat: Arc<dyn Strategy> = match choice {
            prefilter::Choice::Memchr(pre) => Pre::new(pre),
            prefilter::Choice::Memchr2(pre) => Pre::new(pre),
            prefilter::Choice::Memchr3(pre) => Pre::new(pre),
            prefilter::Choice::Memmem(pre) => Pre::new(pre),
            prefilter::Choice::Teddy(pre) => Pre::new(pre),
            prefilter::Choice::ByteSet(pre) => Pre::new(pre),
            prefilter::Choice::AhoCorasick(pre) => Pre::new(pre),
        };
        Some(strat)
    }
    fn from_alternation_literals(
        info: &RegexInfo,
        hirs: &[&Hir],
    ) -> Option<Arc<dyn Strategy>> {}
}
impl Choice {
    pub(crate) fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Choice> {
        if needles.len() == 0 {
            debug!("prefilter building failed: found empty set of literals");
            return None;
        }
        if needles.iter().any(|n| n.as_ref().is_empty()) {
            debug!("prefilter building failed: literals match empty string");
            return None;
        }
        if let Some(pre) = Memchr::new(kind, needles) {
            debug!("prefilter built: memchr");
            return Some(Choice::Memchr(pre));
        }
        if let Some(pre) = Memchr2::new(kind, needles) {
            debug!("prefilter built: memchr2");
            return Some(Choice::Memchr2(pre));
        }
        if let Some(pre) = Memchr3::new(kind, needles) {
            debug!("prefilter built: memchr3");
            return Some(Choice::Memchr3(pre));
        }
        if let Some(pre) = Memmem::new(kind, needles) {
            debug!("prefilter built: memmem");
            return Some(Choice::Memmem(pre));
        }
        if let Some(pre) = Teddy::new(kind, needles) {
            debug!("prefilter built: teddy");
            return Some(Choice::Teddy(pre));
        }
        if let Some(pre) = ByteSet::new(kind, needles) {
            debug!("prefilter built: byteset");
            return Some(Choice::ByteSet(pre));
        }
        if let Some(pre) = AhoCorasick::new(kind, needles) {
            debug!("prefilter built: aho-corasick");
            return Some(Choice::AhoCorasick(pre));
        }
        debug!("prefilter building failed: no strategy could be found");
        None
    }
}
impl<P: PrefilterI> Pre<P> {
    fn new(pre: P) -> Arc<dyn Strategy> {
        let group_info = GroupInfo::new([[None::<&str>]]).unwrap();
        Arc::new(Pre { pre, group_info })
    }
}
impl RegexInfo {
    fn new(config: Config, hirs: &[&Hir]) -> RegexInfo {}
    pub(crate) fn config(&self) -> &Config {
        &self.0.config
    }
    pub(crate) fn props(&self) -> &[hir::Properties] {
        &self.0.props
    }
    pub(crate) fn props_union(&self) -> &hir::Properties {}
    pub(crate) fn pattern_len(&self) -> usize {
        self.props().len()
    }
    pub(crate) fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_anchored_start(&self, input: &Input<'_>) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_always_anchored_start(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_always_anchored_end(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_impossible(&self, input: &Input<'_>) -> bool {}
}
impl Config {
    pub fn new() -> Config {}
    pub fn match_kind(self, kind: MatchKind) -> Config {}
    pub fn utf8_empty(self, yes: bool) -> Config {}
    pub fn auto_prefilter(self, yes: bool) -> Config {}
    pub fn prefilter(self, pre: Option<Prefilter>) -> Config {}
    pub fn which_captures(mut self, which_captures: WhichCaptures) -> Config {}
    pub fn nfa_size_limit(self, limit: Option<usize>) -> Config {}
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
    pub fn get_utf8_empty(&self) -> bool {}
    pub fn get_auto_prefilter(&self) -> bool {}
    pub fn get_prefilter(&self) -> Option<&Prefilter> {}
    pub fn get_which_captures(&self) -> WhichCaptures {}
    pub fn get_nfa_size_limit(&self) -> Option<usize> {}
    pub fn get_onepass_size_limit(&self) -> Option<usize> {}
    pub fn get_hybrid_cache_capacity(&self) -> usize {}
    pub fn get_dfa_size_limit(&self) -> Option<usize> {}
    pub fn get_dfa_state_limit(&self) -> Option<usize> {}
    pub fn get_byte_classes(&self) -> bool {}
    pub fn get_line_terminator(&self) -> u8 {}
    pub fn get_hybrid(&self) -> bool {}
    pub fn get_dfa(&self) -> bool {}
    pub fn get_onepass(&self) -> bool {}
    pub fn get_backtrack(&self) -> bool {}
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
