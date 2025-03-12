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
    fn next(&mut self) -> Option<usize> {
        loop {
            let id = self.it.next()?;
            if self.patset.contains(PatternID::new_unchecked(id)) {
                return Some(id);
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {}
}
