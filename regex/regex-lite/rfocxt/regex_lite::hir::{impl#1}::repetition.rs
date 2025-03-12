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
pub(crate) struct Repetition {
    /// The minimum range of the repetition.
    ///
    /// Note that special cases like `?`, `+` and `*` all get translated into
    /// the ranges `{0,1}`, `{1,}` and `{0,}`, respectively.
    ///
    /// When `min` is zero, this expression can match the empty string
    /// regardless of what its sub-expression is.
    pub(crate) min: u32,
    /// The maximum range of the repetition.
    ///
    /// Note that when `max` is `None`, `min` acts as a lower bound but where
    /// there is no upper bound. For something like `x{5}` where the min and
    /// max are equivalent, `min` will be set to `5` and `max` will be set to
    /// `Some(5)`.
    pub(crate) max: Option<u32>,
    /// Whether this repetition operator is greedy or not. A greedy operator
    /// will match as much as it can. A non-greedy operator will match as
    /// little as it can.
    ///
    /// Typically, operators are greedy by default and are only non-greedy when
    /// a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is
    /// not. However, this can be inverted via the `U` "ungreedy" flag.
    pub(crate) greedy: bool,
    /// The expression being repeated.
    pub(crate) sub: Box<Hir>,
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
    fn repetition(rep: Repetition) -> Hir {
        if rep.min == 0 && rep.max == Some(0) {
            return Hir::empty();
        } else if rep.min == 1 && rep.max == Some(1) {
            return *rep.sub;
        }
        let is_start_anchored = rep.min > 0 && rep.sub.is_start_anchored;
        let is_match_empty = rep.min == 0 || rep.sub.is_match_empty;
        let mut static_explicit_captures_len = rep.sub.static_explicit_captures_len;
        if rep.min == 0 && static_explicit_captures_len.map_or(false, |len| len > 0) {
            if rep.max == Some(0) {
                static_explicit_captures_len = Some(0);
            } else {
                static_explicit_captures_len = None;
            }
        }
        Hir {
            kind: HirKind::Repetition(rep),
            is_start_anchored,
            is_match_empty,
            static_explicit_captures_len,
        }
    }
    fn capture(cap: Capture) -> Hir {}
    fn concat(mut subs: Vec<Hir>) -> Hir {}
    fn alternation(mut subs: Vec<Hir>) -> Hir {}
}
