use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{Error, RegexSetBuilder};
#[derive(Clone, Debug)]
pub struct SetMatchesIter<'a>(PatternSetIter<'a>);
impl<'a> DoubleEndedIterator for SetMatchesIter<'a> {
    fn next_back(&mut self) -> Option<usize> {
        self.0.next_back().map(|pid| pid.as_usize())
    }
}
