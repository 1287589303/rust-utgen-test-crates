use alloc::string::String;
use regex_automata::{meta, Input, PatternID, PatternSet, PatternSetIter};
use crate::{Error, RegexSetBuilder};
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>>;
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}
#[derive(Clone, Debug)]
pub struct SetMatches(PatternSet);
#[derive(Clone, Debug)]
pub struct SetMatchesIter<'a>(PatternSetIter<'a>);
#[derive(Clone, Debug)]
pub struct SetMatchesIter<'a>(PatternSetIter<'a>);
impl<'a> IntoIterator for &'a SetMatches {
    type IntoIter = SetMatchesIter<'a>;
    type Item = usize;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl SetMatches {
    #[inline]
    pub fn matched_any(&self) -> bool {}
    pub fn matched_all(&self) -> bool {}
    #[inline]
    pub fn matched(&self, index: usize) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn iter(&self) -> SetMatchesIter<'_> {
        SetMatchesIter(self.0.iter())
    }
}
