pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
#[derive(Clone, Debug)]
pub struct RegexBuilder {
    builder: Builder,
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
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {
        RegexBuilder::new(re).build()
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
            builder: Builder::new([pattern]),
        }
    }
    pub fn build(&self) -> Result<Regex, Error> {
        self.builder.build_one_string()
    }
    pub fn unicode(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn line_terminator(&mut self, byte: u8) -> &mut RegexBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn size_limit(&mut self, bytes: usize) -> &mut RegexBuilder {}
    pub fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexBuilder {}
    pub fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder {}
}
