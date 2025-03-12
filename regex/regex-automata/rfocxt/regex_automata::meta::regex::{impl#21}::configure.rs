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
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
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
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
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
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
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
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
impl Builder {
    pub fn new() -> Builder {}
    pub fn build(&self, pattern: &str) -> Result<Regex, BuildError> {}
    pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<Regex, BuildError> {}
    pub fn build_from_hir(&self, hir: &Hir) -> Result<Regex, BuildError> {}
    pub fn build_many_from_hir<H: Borrow<Hir>>(
        &self,
        hirs: &[H],
    ) -> Result<Regex, BuildError> {}
    pub fn configure(&mut self, config: Config) -> &mut Builder {
        self.config = self.config.overwrite(config);
        self
    }
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {}
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
    pub(crate) fn overwrite(&self, o: Config) -> Config {
        Config {
            match_kind: o.match_kind.or(self.match_kind),
            utf8_empty: o.utf8_empty.or(self.utf8_empty),
            autopre: o.autopre.or(self.autopre),
            pre: o.pre.or_else(|| self.pre.clone()),
            which_captures: o.which_captures.or(self.which_captures),
            nfa_size_limit: o.nfa_size_limit.or(self.nfa_size_limit),
            onepass_size_limit: o.onepass_size_limit.or(self.onepass_size_limit),
            hybrid_cache_capacity: o
                .hybrid_cache_capacity
                .or(self.hybrid_cache_capacity),
            hybrid: o.hybrid.or(self.hybrid),
            dfa: o.dfa.or(self.dfa),
            dfa_size_limit: o.dfa_size_limit.or(self.dfa_size_limit),
            dfa_state_limit: o.dfa_state_limit.or(self.dfa_state_limit),
            onepass: o.onepass.or(self.onepass),
            backtrack: o.backtrack.or(self.backtrack),
            byte_classes: o.byte_classes.or(self.byte_classes),
            line_terminator: o.line_terminator.or(self.line_terminator),
        }
    }
}
