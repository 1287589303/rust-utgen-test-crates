use alloc::{
    borrow::Cow, boxed::Box, string::String, string::ToString, sync::Arc, vec, vec::Vec,
};
use crate::{
    error::Error, hir::{self, Hir},
    int::NonMaxUsize, interpolate, nfa::{self, NFA},
    pikevm::{self, Cache, PikeVM},
    pool::CachePool,
};
pub struct Regex {
    pikevm: Arc<PikeVM>,
    pool: CachePool,
}
#[derive(Debug)]
pub struct RegexBuilder {
    pattern: String,
    hir_config: hir::Config,
    nfa_config: nfa::Config,
}
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    msg: &'static str,
}
impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, Error> {
        RegexBuilder::new(pattern).build()
    }
    #[inline]
    pub fn is_match(&self, haystack: &str) -> bool {}
    #[inline]
    pub fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>> {}
    #[inline]
    pub fn find_iter<'r, 'h>(&'r self, haystack: &'h str) -> Matches<'r, 'h> {}
    #[inline]
    pub fn captures<'h>(&self, haystack: &'h str) -> Option<Captures<'h>> {}
    #[inline]
    pub fn captures_iter<'r, 'h>(&'r self, haystack: &'h str) -> CaptureMatches<'r, 'h> {}
    #[inline]
    pub fn split<'r, 'h>(&'r self, haystack: &'h str) -> Split<'r, 'h> {}
    #[inline]
    pub fn splitn<'r, 'h>(&'r self, haystack: &'h str, limit: usize) -> SplitN<'r, 'h> {}
    #[inline]
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {}
    #[inline]
    pub fn replace_all<'h, R: Replacer>(
        &self,
        haystack: &'h str,
        rep: R,
    ) -> Cow<'h, str> {}
    #[inline]
    pub fn replacen<'h, R: Replacer>(
        &self,
        haystack: &'h str,
        limit: usize,
        mut rep: R,
    ) -> Cow<'h, str> {}
}
impl RegexBuilder {
    pub fn new(pattern: &str) -> RegexBuilder {
        RegexBuilder {
            pattern: pattern.to_string(),
            hir_config: hir::Config::default(),
            nfa_config: nfa::Config::default(),
        }
    }
    pub fn build(&self) -> Result<Regex, Error> {
        let hir = Hir::parse(self.hir_config, &self.pattern)?;
        let nfa = NFA::new(self.nfa_config, self.pattern.clone(), &hir)?;
        let pikevm = Arc::new(PikeVM::new(nfa));
        let pool = {
            let pikevm = Arc::clone(&pikevm);
            let create = Box::new(move || Cache::new(&pikevm));
            CachePool::new(create)
        };
        Ok(Regex { pikevm, pool })
    }
    pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn size_limit(&mut self, limit: usize) -> &mut RegexBuilder {}
    pub fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder {}
}
