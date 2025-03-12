use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{bytes::RegexSetBuilder, Error};
#[derive(Clone, Debug)]
pub struct SetMatches(PatternSet);
#[derive(Debug)]
pub struct SetMatchesIntoIter {
    patset: PatternSet,
    it: core::ops::Range<usize>,
}
#[derive(Debug)]
pub struct SetMatchesIntoIter {
    patset: PatternSet,
    it: core::ops::Range<usize>,
}
impl IntoIterator for SetMatches {
    type IntoIter = SetMatchesIntoIter;
    type Item = usize;
    fn into_iter(self) -> Self::IntoIter {
        let it = 0..self.0.capacity();
        SetMatchesIntoIter {
            patset: self.0,
            it,
        }
    }
}
