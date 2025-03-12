pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
#[derive(Debug)]
pub struct Matches<'r, 'h> {
    haystack: &'h str,
    it: meta::FindMatches<'r, 'h>,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h [u8],
    start: usize,
    end: usize,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
impl<'r, 'h> Iterator for Matches<'r, 'h> {
    type Item = Match<'h>;
    #[inline]
    fn next(&mut self) -> Option<Match<'h>> {}
    #[inline]
    fn count(self) -> usize {
        self.it.count()
    }
}
