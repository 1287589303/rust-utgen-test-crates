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
    pub fn kind(&self) -> &HirKind {
        &self.kind
    }
    pub fn into_kind(mut self) -> HirKind {
        core::mem::replace(&mut self.kind, HirKind::Empty)
    }
    pub fn properties(&self) -> &Properties {}
    fn into_parts(mut self) -> (HirKind, Properties) {}
}
fn lift_common_prefix(hirs: Vec<Hir>) -> Result<Hir, Vec<Hir>> {
    if hirs.len() <= 1 {
        return Err(hirs);
    }
    let mut prefix = match hirs[0].kind() {
        HirKind::Concat(ref xs) => &**xs,
        _ => return Err(hirs),
    };
    if prefix.is_empty() {
        return Err(hirs);
    }
    for h in hirs.iter().skip(1) {
        let concat = match h.kind() {
            HirKind::Concat(ref xs) => xs,
            _ => return Err(hirs),
        };
        let common_len = prefix
            .iter()
            .zip(concat.iter())
            .take_while(|(x, y)| x == y)
            .count();
        prefix = &prefix[..common_len];
        if prefix.is_empty() {
            return Err(hirs);
        }
    }
    let len = prefix.len();
    assert_ne!(0, len);
    let mut prefix_concat = vec![];
    let mut suffix_alts = vec![];
    for h in hirs {
        let mut concat = match h.into_kind() {
            HirKind::Concat(xs) => xs,
            _ => unreachable!(),
        };
        suffix_alts.push(Hir::concat(concat.split_off(len)));
        if prefix_concat.is_empty() {
            prefix_concat = concat;
        }
    }
    let mut concat = prefix_concat;
    concat.push(Hir::alternation(suffix_alts));
    Ok(Hir::concat(concat))
}
