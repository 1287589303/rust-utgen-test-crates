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
#[derive(Clone, Eq, PartialEq)]
pub struct Literal(pub Box<[u8]>);
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
    pub fn literal<B: Into<Box<[u8]>>>(lit: B) -> Hir {
        let bytes = lit.into();
        if bytes.is_empty() {
            return Hir::empty();
        }
        let lit = Literal(bytes);
        let props = Properties::literal(&lit);
        Hir {
            kind: HirKind::Literal(lit),
            props,
        }
    }
    #[inline]
    pub fn class(class: Class) -> Hir {}
    #[inline]
    pub fn look(look: Look) -> Hir {}
    #[inline]
    pub fn repetition(mut rep: Repetition) -> Hir {}
    #[inline]
    pub fn capture(capture: Capture) -> Hir {}
    pub fn concat(subs: Vec<Hir>) -> Hir {}
    pub fn alternation(subs: Vec<Hir>) -> Hir {}
    #[inline]
    pub fn dot(dot: Dot) -> Hir {}
}
impl Properties {
    fn empty() -> Properties {}
    fn literal(lit: &Literal) -> Properties {
        let inner = PropertiesI {
            minimum_len: Some(lit.0.len()),
            maximum_len: Some(lit.0.len()),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::empty(),
            look_set_suffix: LookSet::empty(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: core::str::from_utf8(&lit.0).is_ok(),
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: true,
            alternation_literal: true,
        };
        Properties(Box::new(inner))
    }
    fn class(class: &Class) -> Properties {}
    fn look(look: Look) -> Properties {}
    fn repetition(rep: &Repetition) -> Properties {}
    fn capture(capture: &Capture) -> Properties {}
    fn concat(concat: &[Hir]) -> Properties {}
    fn alternation(alts: &[Hir]) -> Properties {}
}
