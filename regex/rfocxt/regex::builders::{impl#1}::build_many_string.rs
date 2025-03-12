use alloc::{
    string::{String, ToString},
    sync::Arc, vec, vec::Vec,
};
use regex_automata::{meta, nfa::thompson::WhichCaptures, util::syntax, MatchKind};
use crate::error::Error;
#[derive(Clone, Debug)]
struct Builder {
    pats: Vec<String>,
    metac: meta::Config,
    syntaxc: syntax::Config,
}
#[derive(Clone)]
pub struct RegexSet {
    pub(crate) meta: meta::Regex,
    pub(crate) patterns: alloc::sync::Arc<[String]>,
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
impl Builder {
    fn new<I, S>(patterns: I) -> Builder
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {}
    fn build_one_string(&self) -> Result<crate::Regex, Error> {}
    fn build_one_bytes(&self) -> Result<crate::bytes::Regex, Error> {}
    fn build_many_string(&self) -> Result<crate::RegexSet, Error> {
        let metac = self
            .metac
            .clone()
            .match_kind(MatchKind::All)
            .utf8_empty(true)
            .which_captures(WhichCaptures::None);
        let syntaxc = self.syntaxc.clone().utf8(true);
        let patterns = Arc::from(self.pats.as_slice());
        meta::Builder::new()
            .configure(metac)
            .syntax(syntaxc)
            .build_many(&patterns)
            .map(|meta| crate::RegexSet { meta, patterns })
            .map_err(Error::from_meta_build_error)
    }
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
