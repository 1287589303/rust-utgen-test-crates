use core::{char, cmp};
use alloc::{
    boxed::Box, format, string::{String, ToString},
    vec, vec::Vec,
};
use crate::{
    ast::Span, hir::interval::{Interval, IntervalSet, IntervalSetIter},
    unicode,
};
pub use crate::{
    hir::visitor::{visit, Visitor},
    unicode::CaseFoldError,
};
#[derive(Clone, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    props: Properties,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Properties(Box<PropertiesI>);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Repetition {
    /// The minimum range of the repetition.
    ///
    /// Note that special cases like `?`, `+` and `*` all get translated into
    /// the ranges `{0,1}`, `{1,}` and `{0,}`, respectively.
    ///
    /// When `min` is zero, this expression can match the empty string
    /// regardless of what its sub-expression is.
    pub min: u32,
    /// The maximum range of the repetition.
    ///
    /// Note that when `max` is `None`, `min` acts as a lower bound but where
    /// there is no upper bound. For something like `x{5}` where the min and
    /// max are equivalent, `min` will be set to `5` and `max` will be set to
    /// `Some(5)`.
    pub max: Option<u32>,
    /// Whether this repetition operator is greedy or not. A greedy operator
    /// will match as much as it can. A non-greedy operator will match as
    /// little as it can.
    ///
    /// Typically, operators are greedy by default and are only non-greedy when
    /// a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is
    /// not. However, this can be inverted via the `U` "ungreedy" flag.
    pub greedy: bool,
    /// The expression being repeated.
    pub sub: Box<Hir>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirKind {
    /// The empty regular expression, which matches everything, including the
    /// empty string.
    Empty,
    /// A literalstring that matches exactly these bytes.
    Literal(Literal),
    /// A single character class that matches any of the characters in the
    /// class. A class can either consist of Unicode scalar values as
    /// characters, or it can use bytes.
    ///
    /// A class may be empty. In which case, it matches nothing.
    Class(Class),
    /// A look-around assertion. A look-around match always has zero length.
    Look(Look),
    /// A repetition operation applied to a sub-expression.
    Repetition(Repetition),
    /// A capturing group, which contains a sub-expression.
    Capture(Capture),
    /// A concatenation of expressions.
    ///
    /// A concatenation matches only if each of its sub-expressions match one
    /// after the other.
    ///
    /// Concatenations are guaranteed by `Hir`'s smart constructors to always
    /// have at least two sub-expressions.
    Concat(Vec<Hir>),
    /// An alternation of expressions.
    ///
    /// An alternation matches only if at least one of its sub-expressions
    /// match. If multiple sub-expressions match, then the leftmost is
    /// preferred.
    ///
    /// Alternations are guaranteed by `Hir`'s smart constructors to always
    /// have at least two sub-expressions.
    Alternation(Vec<Hir>),
}
impl Hir {
    #[inline]
    pub fn empty() -> Hir {
        let props = Properties::empty();
        Hir { kind: HirKind::Empty, props }
    }
    #[inline]
    pub fn fail() -> Hir {}
    #[inline]
    pub fn literal<B: Into<Box<[u8]>>>(lit: B) -> Hir {}
    #[inline]
    pub fn class(class: Class) -> Hir {}
    #[inline]
    pub fn look(look: Look) -> Hir {}
    #[inline]
    pub fn repetition(mut rep: Repetition) -> Hir {
        if rep.sub.properties().maximum_len() == Some(0) {
            rep.min = cmp::min(rep.min, 1);
            rep.max = rep.max.map(|n| cmp::min(n, 1)).or(Some(1));
        }
        if rep.min == 0 && rep.max == Some(0) {
            return Hir::empty();
        } else if rep.min == 1 && rep.max == Some(1) {
            return *rep.sub;
        }
        let props = Properties::repetition(&rep);
        Hir {
            kind: HirKind::Repetition(rep),
            props,
        }
    }
    #[inline]
    pub fn capture(capture: Capture) -> Hir {}
    pub fn concat(subs: Vec<Hir>) -> Hir {}
    pub fn alternation(subs: Vec<Hir>) -> Hir {}
    #[inline]
    pub fn dot(dot: Dot) -> Hir {}
}
impl Properties {
    fn empty() -> Properties {}
    fn literal(lit: &Literal) -> Properties {}
    fn class(class: &Class) -> Properties {}
    fn look(look: Look) -> Properties {}
    fn repetition(rep: &Repetition) -> Properties {
        let p = rep.sub.properties();
        let minimum_len = p
            .minimum_len()
            .map(|child_min| {
                let rep_min = usize::try_from(rep.min).unwrap_or(usize::MAX);
                child_min.saturating_mul(rep_min)
            });
        let maximum_len = rep
            .max
            .and_then(|rep_max| {
                let rep_max = usize::try_from(rep_max).ok()?;
                let child_max = p.maximum_len()?;
                child_max.checked_mul(rep_max)
            });
        let mut inner = PropertiesI {
            minimum_len,
            maximum_len,
            look_set: p.look_set(),
            look_set_prefix: LookSet::empty(),
            look_set_suffix: LookSet::empty(),
            look_set_prefix_any: p.look_set_prefix_any(),
            look_set_suffix_any: p.look_set_suffix_any(),
            utf8: p.is_utf8(),
            explicit_captures_len: p.explicit_captures_len(),
            static_explicit_captures_len: p.static_explicit_captures_len(),
            literal: false,
            alternation_literal: false,
        };
        if rep.min > 0 {
            inner.look_set_prefix = p.look_set_prefix();
            inner.look_set_suffix = p.look_set_suffix();
        }
        if rep.min == 0
            && inner.static_explicit_captures_len.map_or(false, |len| len > 0)
        {
            if rep.max == Some(0) {
                inner.static_explicit_captures_len = Some(0);
            } else {
                inner.static_explicit_captures_len = None;
            }
        }
        Properties(Box::new(inner))
    }
    fn capture(capture: &Capture) -> Properties {}
    fn concat(concat: &[Hir]) -> Properties {}
    fn alternation(alts: &[Hir]) -> Properties {}
}
