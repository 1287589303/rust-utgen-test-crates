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
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
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
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
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
    pub fn configure(&mut self, config: Config) -> &mut Builder {}
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {
        config.apply_ast(&mut self.ast);
        config.apply_hir(&mut self.hir);
        self
    }
}
impl Config {
    pub fn new() -> Config {}
    pub fn case_insensitive(mut self, yes: bool) -> Config {}
    pub fn multi_line(mut self, yes: bool) -> Config {}
    pub fn dot_matches_new_line(mut self, yes: bool) -> Config {}
    pub fn crlf(mut self, yes: bool) -> Config {}
    pub fn line_terminator(mut self, byte: u8) -> Config {}
    pub fn swap_greed(mut self, yes: bool) -> Config {}
    pub fn ignore_whitespace(mut self, yes: bool) -> Config {}
    pub fn unicode(mut self, yes: bool) -> Config {}
    pub fn utf8(mut self, yes: bool) -> Config {}
    pub fn nest_limit(mut self, limit: u32) -> Config {}
    pub fn octal(mut self, yes: bool) -> Config {}
    pub fn get_unicode(&self) -> bool {}
    pub fn get_case_insensitive(&self) -> bool {}
    pub fn get_multi_line(&self) -> bool {}
    pub fn get_dot_matches_new_line(&self) -> bool {}
    pub fn get_crlf(&self) -> bool {}
    pub fn get_line_terminator(&self) -> u8 {}
    pub fn get_swap_greed(&self) -> bool {}
    pub fn get_ignore_whitespace(&self) -> bool {}
    pub fn get_utf8(&self) -> bool {}
    pub fn get_nest_limit(&self) -> u32 {}
    pub fn get_octal(&self) -> bool {}
    pub(crate) fn apply(&self, builder: &mut ParserBuilder) {}
    pub(crate) fn apply_ast(&self, builder: &mut ast::parse::ParserBuilder) {
        builder
            .ignore_whitespace(self.ignore_whitespace)
            .nest_limit(self.nest_limit)
            .octal(self.octal);
    }
    pub(crate) fn apply_hir(&self, builder: &mut hir::translate::TranslatorBuilder) {
        builder
            .unicode(self.unicode)
            .case_insensitive(self.case_insensitive)
            .multi_line(self.multi_line)
            .crlf(self.crlf)
            .dot_matches_new_line(self.dot_matches_new_line)
            .line_terminator(self.line_terminator)
            .swap_greed(self.swap_greed)
            .utf8(self.utf8);
    }
}
