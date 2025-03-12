pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
#[derive(Debug)]
pub struct CaptureMatches<'r, 'h> {
    haystack: &'h str,
    it: meta::CapturesMatches<'r, 'h>,
}
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
impl<'r, 'h> Iterator for CaptureMatches<'r, 'h> {
    type Item = Captures<'h>;
    #[inline]
    fn next(&mut self) -> Option<Captures<'h>> {
        let static_captures_len = self.it.regex().static_captures_len();
        self.it
            .next()
            .map(|caps| Captures {
                haystack: self.haystack,
                caps,
                static_captures_len,
            })
    }
    #[inline]
    fn count(self) -> usize {}
}
