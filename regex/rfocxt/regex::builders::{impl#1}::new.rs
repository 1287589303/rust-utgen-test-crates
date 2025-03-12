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
impl Builder {
    fn new<I, S>(patterns: I) -> Builder
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let mut b = Builder::default();
        b.pats.extend(patterns.into_iter().map(|p| p.as_ref().to_string()));
        b
    }
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
    fn size_limit(&mut self, limit: usize) -> &mut Builder {}
    fn dfa_size_limit(&mut self, limit: usize) -> &mut Builder {}
    fn nest_limit(&mut self, limit: u32) -> &mut Builder {}
}
impl Default for Builder {
    fn default() -> Builder {
        let metac = meta::Config::new()
            .nfa_size_limit(Some(10 * (1 << 20)))
            .hybrid_cache_capacity(2 * (1 << 20));
        Builder {
            pats: vec![],
            metac,
            syntaxc: syntax::Config::default(),
        }
    }
}
