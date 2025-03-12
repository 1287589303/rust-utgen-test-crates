use crate::{
    hybrid::{
        dfa::{Cache, OverlappingState, DFA},
        id::LazyStateID,
    },
    util::{prefilter::Prefilter, search::{HalfMatch, Input, MatchError, Span}},
};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
impl MatchError {
    pub fn new(kind: MatchErrorKind) -> MatchError {}
    pub fn kind(&self) -> &MatchErrorKind {}
    pub fn quit(byte: u8, offset: usize) -> MatchError {}
    pub fn gave_up(offset: usize) -> MatchError {
        MatchError::new(MatchErrorKind::GaveUp { offset })
    }
    pub fn haystack_too_long(len: usize) -> MatchError {}
    pub fn unsupported_anchored(mode: Anchored) -> MatchError {}
}
#[cfg_attr(feature = "perf-inline", inline(always))]
fn gave_up(offset: usize) -> MatchError {
    MatchError::gave_up(offset)
}
