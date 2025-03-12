use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{bytes::RegexSetBuilder, Error};
#[derive(Clone)]
pub struct RegexSet {
    pub(crate) meta: meta::Regex,
    pub(crate) patterns: alloc::sync::Arc<[String]>,
}
#[derive(Clone, Debug)]
pub struct RegexSetBuilder {
    builder: Builder,
}
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
#[non_exhaustive]
#[derive(Clone, PartialEq)]
pub enum Error {
    /// A syntax error.
    Syntax(String),
    /// The compiled program exceeded the set size
    /// limit. The argument is the size limit imposed by
    /// [`RegexBuilder::size_limit`](crate::RegexBuilder::size_limit). Even
    /// when not configured explicitly, it defaults to a reasonable limit.
    ///
    /// If you're getting this error, it occurred because your regex has been
    /// compiled to an intermediate state that is too big. It is important to
    /// note that exceeding this limit does _not_ mean the regex is too big to
    /// _work_, but rather, the regex is big enough that it may wind up being
    /// surprisingly slow when used in a search. In other words, this error is
    /// meant to be a practical heuristic for avoiding a performance footgun,
    /// and especially so for the case where the regex pattern is coming from
    /// an untrusted source.
    ///
    /// There are generally two ways to move forward if you hit this error.
    /// The first is to find some way to use a smaller regex. The second is to
    /// increase the size limit via `RegexBuilder::size_limit`. However, if
    /// your regex pattern is not from a trusted source, then neither of these
    /// approaches may be appropriate. Instead, you'll have to determine just
    /// how big of a regex you want to allow.
    CompiledTooBig(usize),
}
impl RegexSet {
    pub fn new<I, S>(exprs: I) -> Result<RegexSet, Error>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {}
    pub fn empty() -> RegexSet {
        let empty: [&str; 0] = [];
        RegexSetBuilder::new(empty).build().unwrap()
    }
    #[inline]
    pub fn is_match(&self, haystack: &[u8]) -> bool {}
    #[inline]
    pub fn is_match_at(&self, haystack: &[u8], start: usize) -> bool {}
    #[inline]
    pub fn matches(&self, haystack: &[u8]) -> SetMatches {}
    #[inline]
    pub fn matches_at(&self, haystack: &[u8], start: usize) -> SetMatches {}
    #[inline]
    pub fn matches_read_at(
        &self,
        matches: &mut [bool],
        haystack: &[u8],
        start: usize,
    ) -> bool {}
    #[inline]
    pub fn read_matches_at(
        &self,
        matches: &mut [bool],
        haystack: &[u8],
        start: usize,
    ) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn patterns(&self) -> &[String] {}
}
impl RegexSetBuilder {
    pub fn new<I, S>(patterns: I) -> RegexSetBuilder
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        RegexSetBuilder {
            builder: Builder::new(patterns),
        }
    }
    pub fn build(&self) -> Result<RegexSet, Error> {
        self.builder.build_many_bytes()
    }
    pub fn unicode(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn line_terminator(&mut self, byte: u8) -> &mut RegexSetBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder {}
    pub fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder {}
    pub fn nest_limit(&mut self, limit: u32) -> &mut RegexSetBuilder {}
}
