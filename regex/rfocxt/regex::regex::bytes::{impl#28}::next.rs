pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
#[derive(Clone, Debug)]
pub struct SubCaptureMatches<'c, 'h> {
    haystack: &'h [u8],
    it: captures::CapturesPatternIter<'c>,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h [u8],
    start: usize,
    end: usize,
}
impl<'c, 'h> Iterator for SubCaptureMatches<'c, 'h> {
    type Item = Option<Match<'h>>;
    #[inline]
    fn next(&mut self) -> Option<Option<Match<'h>>> {
        self.it
            .next()
            .map(|group| { group.map(|sp| Match::new(self.haystack, sp.start, sp.end)) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {}
    #[inline]
    fn count(self) -> usize {}
}
