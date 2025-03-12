pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
#[derive(Debug)]
pub struct Split<'r, 'h> {
    haystack: &'h str,
    it: meta::Split<'r, 'h>,
}
#[derive(Debug)]
pub struct Split<'r, 'h> {
    haystack: &'h [u8],
    it: meta::Split<'r, 'h>,
}
impl<'r, 'h> Iterator for Split<'r, 'h> {
    type Item = &'h str;
    #[inline]
    fn next(&mut self) -> Option<&'h str> {
        self.it.next().map(|span| &self.haystack[span])
    }
}
