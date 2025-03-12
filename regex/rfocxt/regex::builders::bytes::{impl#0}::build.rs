use crate::{
    bytes::{Regex, RegexSet},
    error::Error,
};
use super::Builder;
#[derive(Clone, Debug)]
pub struct RegexBuilder {
    builder: Builder,
}
#[derive(Clone, Debug)]
struct Builder {
    pats: Vec<String>,
    metac: meta::Config,
    syntaxc: syntax::Config,
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
impl RegexBuilder {
    pub fn new(pattern: &str) -> RegexBuilder {}
    pub fn build(&self) -> Result<Regex, Error> {
        self.builder.build_one_bytes()
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
impl Builder {
    fn new<I, S>(patterns: I) -> Builder
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {}
    fn build_one_string(&self) -> Result<crate::Regex, Error> {}
    fn build_one_bytes(&self) -> Result<crate::bytes::Regex, Error> {
        assert_eq!(1, self.pats.len());
        let metac = self
            .metac
            .clone()
            .match_kind(MatchKind::LeftmostFirst)
            .utf8_empty(false);
        let syntaxc = self.syntaxc.clone().utf8(false);
        let pattern = Arc::from(self.pats[0].as_str());
        meta::Builder::new()
            .configure(metac)
            .syntax(syntaxc)
            .build(&pattern)
            .map(|meta| crate::bytes::Regex {
                meta,
                pattern,
            })
            .map_err(Error::from_meta_build_error)
    }
    fn build_many_string(&self) -> Result<crate::RegexSet, Error> {}
    fn build_many_bytes(&self) -> Result<crate::bytes::RegexSet, Error> {}
    fn case_insensitive(&mut self, yes: bool) -> &mut Builder {}
    fn multi_line(&mut self, yes: bool) -> &mut Builder {}
    fn dot_matches_new_line(&mut self, yes: bool) -> &mut Builder {}
    fn crlf(&mut self, yes: bool) -> &mut Builder {}
    fn line_terminator(&mut self, byte: u8) -> &mut Builder {}
    fn swap_greed(&mut self, yes: bool) -> &mut Builder {}
    fn ignore_whitespace(&mut self, yes: bool) -> &mut Builder {}
    fn unicode(&mut self, yes: bool) -> &mut Builder {}
    fn octal(&mut self, yes: bool) -> &mut Builder {}
    fn size_limit(&mut self, limit: usize) -> &mut Builder {}
    fn dfa_size_limit(&mut self, limit: usize) -> &mut Builder {}
    fn nest_limit(&mut self, limit: u32) -> &mut Builder {}
}
