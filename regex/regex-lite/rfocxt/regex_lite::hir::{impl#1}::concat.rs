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
    fn fail() -> Hir {}
    fn empty() -> Hir {
        let kind = HirKind::Empty;
        Hir {
            kind,
            is_start_anchored: false,
            is_match_empty: true,
            static_explicit_captures_len: Some(0),
        }
    }
    fn char(ch: char) -> Hir {}
    fn class(class: Class) -> Hir {}
    fn look(look: Look) -> Hir {}
    fn repetition(rep: Repetition) -> Hir {}
    fn capture(cap: Capture) -> Hir {}
    fn concat(mut subs: Vec<Hir>) -> Hir {
        if subs.is_empty() {
            Hir::empty()
        } else if subs.len() == 1 {
            subs.pop().unwrap()
        } else {
            let is_start_anchored = subs[0].is_start_anchored;
            let mut is_match_empty = true;
            let mut static_explicit_captures_len = Some(0usize);
            for sub in subs.iter() {
                is_match_empty = is_match_empty && sub.is_match_empty;
                static_explicit_captures_len = static_explicit_captures_len
                    .and_then(|len1| { Some((len1, sub.static_explicit_captures_len?)) })
                    .and_then(|(len1, len2)| Some(len1.saturating_add(len2)));
            }
            Hir {
                kind: HirKind::Concat(subs),
                is_start_anchored,
                is_match_empty,
                static_explicit_captures_len,
            }
        }
    }
    fn alternation(mut subs: Vec<Hir>) -> Hir {}
}
