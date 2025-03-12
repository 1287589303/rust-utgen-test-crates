use alloc::{
    borrow::Cow, boxed::Box, string::String, string::ToString, sync::Arc, vec, vec::Vec,
};
use crate::{
    error::Error, hir::{self, Hir},
    int::NonMaxUsize, interpolate, nfa::{self, NFA},
    pikevm::{self, Cache, PikeVM},
    pool::CachePool,
};
#[derive(Debug)]
pub struct RegexBuilder {
    pattern: String,
    hir_config: hir::Config,
    nfa_config: nfa::Config,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    pub(crate) size_limit: Option<usize>,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    /// The maximum number of times we're allowed to recurse.
    ///
    /// Note that unlike the regex-syntax parser, we actually use recursion in
    /// this parser for simplicity. My hope is that by setting a conservative
    /// default call limit and providing a way to configure it, that we can
    /// keep this simplification. But if we must, we can re-work the parser to
    /// put the call stack on the heap like regex-syntax does.
    pub(crate) nest_limit: u32,
    /// Various flags that control how a pattern is interpreted.
    pub(crate) flags: Flags,
}
impl RegexBuilder {
    pub fn new(pattern: &str) -> RegexBuilder {}
    pub fn build(&self) -> Result<Regex, Error> {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder {
        self.hir_config.flags.case_insensitive = yes;
        self
    }
    pub fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn size_limit(&mut self, limit: usize) -> &mut RegexBuilder {}
    pub fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder {}
}
