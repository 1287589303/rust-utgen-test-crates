pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
#[derive(Clone, Debug)]
pub struct CaptureLocations(captures::Captures);
pub struct Captures<'h> {
    haystack: &'h str,
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
pub struct Captures<'h> {
    haystack: &'h [u8],
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
impl CaptureLocations {
    #[inline]
    pub fn get(&self, i: usize) -> Option<(usize, usize)> {}
    #[inline]
    pub fn len(&self) -> usize {
        self.0.group_info().group_len(PatternID::ZERO)
    }
    #[inline]
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {}
}
