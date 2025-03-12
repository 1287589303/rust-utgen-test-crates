use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{bytes::RegexSetBuilder, Error};
#[derive(Clone, Debug)]
pub struct SetMatches(PatternSet);
impl SetMatches {
    #[inline]
    pub fn matched_any(&self) -> bool {}
    pub fn matched_all(&self) -> bool {}
    #[inline]
    pub fn matched(&self, index: usize) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {
        self.0.capacity()
    }
    #[inline]
    pub fn iter(&self) -> SetMatchesIter<'_> {}
}
