use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{Error, RegexSetBuilder};
#[derive(Clone, Debug)]
pub struct SetMatchesIter<'a>(PatternSetIter<'a>);
impl<'a> Iterator for SetMatchesIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.0.next().map(|pid| pid.as_usize())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {}
}
