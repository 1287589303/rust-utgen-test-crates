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
impl RegexBuilder {
    pub fn new(pattern: &str) -> RegexBuilder {}
    pub fn build(&self) -> Result<Regex, Error> {}
    pub fn unicode(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn line_terminator(&mut self, byte: u8) -> &mut RegexBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut RegexBuilder {}
    pub fn size_limit(&mut self, bytes: usize) -> &mut RegexBuilder {
        self.builder.size_limit(bytes);
        self
    }
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
    fn build_one_bytes(&self) -> Result<crate::bytes::Regex, Error> {}
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
    fn size_limit(&mut self, limit: usize) -> &mut Builder {
        self.metac = self.metac.clone().nfa_size_limit(Some(limit));
        self
    }
    fn dfa_size_limit(&mut self, limit: usize) -> &mut Builder {}
    fn nest_limit(&mut self, limit: u32) -> &mut Builder {}
}
