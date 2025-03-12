pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
fn no_expansion<T: AsRef<str>>(replacement: &T) -> Option<Cow<'_, str>> {
    let replacement = replacement.as_ref();
    match crate::find_byte::find_byte(b'$', replacement.as_bytes()) {
        Some(_) => None,
        None => Some(Cow::Borrowed(replacement)),
    }
}
pub(crate) fn find_byte(needle: u8, haystack: &[u8]) -> Option<usize> {
    #[cfg(not(feature = "perf-literal"))]
    fn imp(needle: u8, haystack: &[u8]) -> Option<usize> {
        haystack.iter().position(|&b| b == needle)
    }
    #[cfg(feature = "perf-literal")]
    fn imp(needle: u8, haystack: &[u8]) -> Option<usize> {
        memchr::memchr(needle, haystack)
    }
    imp(needle, haystack)
}
