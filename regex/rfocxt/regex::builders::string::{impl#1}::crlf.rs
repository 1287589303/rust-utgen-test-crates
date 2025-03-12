use crate::{error::Error, Regex, RegexSet};
use super::Builder;
#[derive(Clone, Debug)]
pub struct RegexSetBuilder {
    builder: Builder,
}
#[derive(Clone, Debug)]
struct Builder {
    pats: Vec<String>,
    metac: meta::Config,
    syntaxc: syntax::Config,
}
impl RegexSetBuilder {
    pub fn new<I, S>(patterns: I) -> RegexSetBuilder
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {}
    pub fn build(&self) -> Result<RegexSet, Error> {}
    pub fn unicode(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut RegexSetBuilder {
        self.builder.crlf(yes);
        self
    }
    pub fn line_terminator(&mut self, byte: u8) -> &mut RegexSetBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut RegexSetBuilder {}
    pub fn size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder {}
    pub fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder {}
    pub fn nest_limit(&mut self, limit: u32) -> &mut RegexSetBuilder {}
}
impl Builder {
    fn new<I, S>(patterns: I) -> Builder
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {}
    fn build_one_string(&self) -> Result<crate::Regex, Error> {}
    fn build_one_bytes(&self) -> Result<crate::bytes::Regex, Error> {}
    fn build_many_string(&self) -> Result<crate::RegexSet, Error> {}
    fn build_many_bytes(&self) -> Result<crate::bytes::RegexSet, Error> {}
    fn case_insensitive(&mut self, yes: bool) -> &mut Builder {}
    fn multi_line(&mut self, yes: bool) -> &mut Builder {}
    fn dot_matches_new_line(&mut self, yes: bool) -> &mut Builder {}
    fn crlf(&mut self, yes: bool) -> &mut Builder {
        self.syntaxc = self.syntaxc.crlf(yes);
        self
    }
    fn line_terminator(&mut self, byte: u8) -> &mut Builder {}
    fn swap_greed(&mut self, yes: bool) -> &mut Builder {}
    fn ignore_whitespace(&mut self, yes: bool) -> &mut Builder {}
    fn unicode(&mut self, yes: bool) -> &mut Builder {}
    fn octal(&mut self, yes: bool) -> &mut Builder {}
    fn size_limit(&mut self, limit: usize) -> &mut Builder {}
    fn dfa_size_limit(&mut self, limit: usize) -> &mut Builder {}
    fn nest_limit(&mut self, limit: u32) -> &mut Builder {}
}
