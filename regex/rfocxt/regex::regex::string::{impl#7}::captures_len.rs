pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
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
impl Regex {
    #[inline]
    pub fn as_str(&self) -> &str {}
    #[inline]
    pub fn capture_names(&self) -> CaptureNames<'_> {}
    #[inline]
    pub fn captures_len(&self) -> usize {
        self.meta.group_info().group_len(PatternID::ZERO)
    }
    #[inline]
    pub fn static_captures_len(&self) -> Option<usize> {}
    #[inline]
    pub fn capture_locations(&self) -> CaptureLocations {}
    #[inline]
    pub fn locations(&self) -> CaptureLocations {}
}
