use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{bytes::RegexSetBuilder, Error};
#[derive(Debug)]
pub struct SetMatchesIntoIter {
    patset: PatternSet,
    it: core::ops::Range<usize>,
}
impl Iterator for SetMatchesIntoIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {}
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
