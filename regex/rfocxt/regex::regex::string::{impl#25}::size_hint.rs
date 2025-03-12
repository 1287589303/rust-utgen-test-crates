pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
#[derive(Clone, Debug)]
pub struct CaptureNames<'r>(captures::GroupInfoPatternNames<'r>);
impl<'r> Iterator for CaptureNames<'r> {
    type Item = Option<&'r str>;
    #[inline]
    fn next(&mut self) -> Option<Option<&'r str>> {}
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    #[inline]
    fn count(self) -> usize {}
}
