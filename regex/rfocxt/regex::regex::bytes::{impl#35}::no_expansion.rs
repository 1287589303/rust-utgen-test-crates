pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>> {
        None
    }
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}
impl Replacer for Vec<u8> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {}
    fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>> {
        no_expansion(self)
    }
}
