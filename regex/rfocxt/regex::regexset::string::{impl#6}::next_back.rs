use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{Error, RegexSetBuilder};
#[derive(Debug)]
pub struct SetMatchesIntoIter {
    patset: PatternSet,
    it: core::ops::Range<usize>,
}
impl DoubleEndedIterator for SetMatchesIntoIter {
    fn next_back(&mut self) -> Option<usize> {
        loop {
            let id = self.it.next_back()?;
            if self.patset.contains(PatternID::new_unchecked(id)) {
                return Some(id);
            }
        }
    }
}
