pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
#[derive(Debug)]
pub struct SplitN<'r, 'h> {
    haystack: &'h str,
    it: meta::SplitN<'r, 'h>,
}
#[derive(Debug)]
pub struct SplitN<'r, 'h> {
    haystack: &'h [u8],
    it: meta::SplitN<'r, 'h>,
}
impl<'r, 'h> Iterator for SplitN<'r, 'h> {
    type Item = &'h str;
    #[inline]
    fn next(&mut self) -> Option<&'h str> {
        self.it.next().map(|span| &self.haystack[span])
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {}
}
