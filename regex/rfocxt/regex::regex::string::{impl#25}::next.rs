pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
#[derive(Clone, Debug)]
pub struct CaptureNames<'r>(captures::GroupInfoPatternNames<'r>);
impl<'r> Iterator for CaptureNames<'r> {
    type Item = Option<&'r str>;
    #[inline]
    fn next(&mut self) -> Option<Option<&'r str>> {
        self.0.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {}
    #[inline]
    fn count(self) -> usize {}
}
