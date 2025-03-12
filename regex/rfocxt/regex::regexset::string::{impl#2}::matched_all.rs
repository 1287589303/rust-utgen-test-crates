use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{Error, RegexSetBuilder};
#[derive(Clone, Debug)]
pub struct SetMatches(PatternSet);
impl SetMatches {
    #[inline]
    pub fn matched_any(&self) -> bool {}
    pub fn matched_all(&self) -> bool {
        self.0.is_full()
    }
    #[inline]
    pub fn matched(&self, index: usize) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn iter(&self) -> SetMatchesIter<'_> {}
}
