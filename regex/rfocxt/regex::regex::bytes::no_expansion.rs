pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
fn no_expansion<T: AsRef<[u8]>>(replacement: &T) -> Option<Cow<'_, [u8]>> {
    let replacement = replacement.as_ref();
    match crate::find_byte::find_byte(b'$', replacement) {
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
