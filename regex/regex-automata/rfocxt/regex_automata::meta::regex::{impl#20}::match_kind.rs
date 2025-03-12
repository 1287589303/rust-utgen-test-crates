type CachePool = Pool<Cache, CachePoolFn>;
type CachePoolGuard<'a> = PoolGuard<'a, Cache, CachePoolFn>;
type CachePoolFn = Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>;
use core::{borrow::Borrow, panic::{RefUnwindSafe, UnwindSafe}};
use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use regex_syntax::{ast, hir::{self, Hir}};
use crate::{
    meta::{
        error::BuildError, strategy::{self, Strategy},
        wrappers,
    },
    nfa::thompson::WhichCaptures,
    util::{
        captures::{Captures, GroupInfo},
        iter, pool::{Pool, PoolGuard},
        prefilter::Prefilter, primitives::{NonMaxUsize, PatternID},
        search::{HalfMatch, Input, Match, MatchKind, PatternSet, Span},
    },
};
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
impl Config {
    pub fn new() -> Config {}
    pub fn match_kind(self, kind: MatchKind) -> Config {
        Config {
            match_kind: Some(kind),
            ..self
        }
    }
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
    pub fn get_match_kind(&self) -> MatchKind {}
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
