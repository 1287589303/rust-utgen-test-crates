pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
#[derive(Debug)]
pub struct SplitN<'r, 'h> {
    haystack: &'h [u8],
    it: meta::SplitN<'r, 'h>,
}
#[derive(Debug)]
pub struct SplitN<'r, 'h> {
    haystack: &'h str,
    it: meta::SplitN<'r, 'h>,
}
impl<'r, 'h> Iterator for SplitN<'r, 'h> {
    type Item = &'h [u8];
    #[inline]
    fn next(&mut self) -> Option<&'h [u8]> {}
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
