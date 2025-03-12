use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Hir {
    kind: HirKind,
    is_start_anchored: bool,
    is_match_empty: bool,
    static_explicit_captures_len: Option<usize>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum HirKind {
    Empty,
    Char(char),
    Class(Class),
    Look(Look),
    Repetition(Repetition),
    Capture(Capture),
    Concat(Vec<Hir>),
    Alternation(Vec<Hir>),
}
impl Hir {
    pub(crate) fn parse(config: Config, pattern: &str) -> Result<Hir, Error> {}
    pub(crate) fn kind(&self) -> &HirKind {}
    pub(crate) fn is_start_anchored(&self) -> bool {}
    pub(crate) fn is_match_empty(&self) -> bool {}
    pub(crate) fn static_explicit_captures_len(&self) -> Option<usize> {}
    fn fail() -> Hir {
        let kind = HirKind::Class(Class { ranges: vec![] });
        Hir {
            kind,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: Some(0),
        }
    }
    fn empty() -> Hir {}
    fn char(ch: char) -> Hir {}
    fn class(class: Class) -> Hir {}
    fn look(look: Look) -> Hir {}
    fn repetition(rep: Repetition) -> Hir {}
    fn capture(cap: Capture) -> Hir {}
    fn concat(mut subs: Vec<Hir>) -> Hir {}
    fn alternation(mut subs: Vec<Hir>) -> Hir {
        if subs.is_empty() {
            Hir::fail()
        } else if subs.len() == 1 {
            subs.pop().unwrap()
        } else {
            let mut it = subs.iter().peekable();
            let mut is_start_anchored = it
                .peek()
                .map_or(false, |sub| sub.is_start_anchored);
            let mut is_match_empty = it.peek().map_or(false, |sub| sub.is_match_empty);
            let mut static_explicit_captures_len = it
                .peek()
                .and_then(|sub| sub.static_explicit_captures_len);
            for sub in it {
                is_start_anchored = is_start_anchored && sub.is_start_anchored;
                is_match_empty = is_match_empty || sub.is_match_empty;
                if static_explicit_captures_len != sub.static_explicit_captures_len {
                    static_explicit_captures_len = None;
                }
            }
            Hir {
                kind: HirKind::Alternation(subs),
                is_start_anchored,
                is_match_empty,
                static_explicit_captures_len,
            }
        }
    }
}
