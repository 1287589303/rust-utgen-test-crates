use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{Error, RegexSetBuilder};
#[derive(Clone)]
pub struct RegexSet {
    pub(crate) meta: meta::Regex,
    pub(crate) patterns: alloc::sync::Arc<[String]>,
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
impl core::fmt::Debug for RegexSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RegexSet({:?})", self.patterns())
    }
}
impl RegexSet {
    pub fn new<I, S>(exprs: I) -> Result<RegexSet, Error>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {}
    pub fn empty() -> RegexSet {}
    #[inline]
    pub fn is_match(&self, haystack: &str) -> bool {}
    #[inline]
    pub fn is_match_at(&self, haystack: &str, start: usize) -> bool {}
    #[inline]
    pub fn matches(&self, haystack: &str) -> SetMatches {}
    #[inline]
    pub fn matches_at(&self, haystack: &str, start: usize) -> SetMatches {}
    #[inline]
    pub fn matches_read_at(
        &self,
        matches: &mut [bool],
        haystack: &str,
        start: usize,
    ) -> bool {}
    #[inline]
    pub fn read_matches_at(
        &self,
        matches: &mut [bool],
        haystack: &str,
        start: usize,
    ) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn patterns(&self) -> &[String] {
        &self.patterns
    }
}
