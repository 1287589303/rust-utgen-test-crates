pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
impl core::fmt::Debug for Regex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Regex").field(&self.as_str()).finish()
    }
}
impl Regex {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.pattern
    }
    #[inline]
    pub fn capture_names(&self) -> CaptureNames<'_> {}
    #[inline]
    pub fn captures_len(&self) -> usize {}
    #[inline]
    pub fn static_captures_len(&self) -> Option<usize> {}
    #[inline]
    pub fn capture_locations(&self) -> CaptureLocations {}
    #[inline]
    pub fn locations(&self) -> CaptureLocations {}
}
