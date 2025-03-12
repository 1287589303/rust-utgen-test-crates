pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
#[derive(Debug)]
pub struct Split<'r, 'h> {
    haystack: &'h [u8],
    it: meta::Split<'r, 'h>,
}
#[derive(Debug)]
pub struct Split<'r, 'h> {
    haystack: &'h str,
    it: meta::Split<'r, 'h>,
}
impl<'r, 'h> Iterator for Split<'r, 'h> {
    type Item = &'h [u8];
    #[inline]
    fn next(&mut self) -> Option<&'h [u8]> {
        self.it.next().map(|span| &self.haystack[span])
    }
}
