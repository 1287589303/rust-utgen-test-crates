use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{bytes::RegexSetBuilder, Error};
#[derive(Clone, Debug)]
pub struct SetMatches(PatternSet);
#[derive(Clone, Debug)]
pub struct SetMatchesIter<'a>(PatternSetIter<'a>);
impl SetMatches {
    #[inline]
    pub fn matched_any(&self) -> bool {}
    pub fn matched_all(&self) -> bool {}
    #[inline]
    pub fn matched(&self, index: usize) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn iter(&self) -> SetMatchesIter<'_> {
        SetMatchesIter(self.0.iter())
    }
}
