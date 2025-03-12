pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
#[derive(Clone, Debug)]
pub struct CaptureLocations(captures::Captures);
pub struct Captures<'h> {
    haystack: &'h [u8],
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
pub struct Captures<'h> {
    haystack: &'h str,
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
impl CaptureLocations {
    #[inline]
    pub fn get(&self, i: usize) -> Option<(usize, usize)> {
        self.0.get_group(i).map(|sp| (sp.start, sp.end))
    }
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
        self.get(i)
    }
}
