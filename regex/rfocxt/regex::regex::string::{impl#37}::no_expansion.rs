pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
        None
    }
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}
#[derive(Debug)]
pub struct ReplacerRef<'a, R: ?Sized>(&'a mut R);
impl<'a, R: Replacer + ?Sized + 'a> Replacer for ReplacerRef<'a, R> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {}
    fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
        self.0.no_expansion()
    }
}
