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
    pub fn into_kind(mut self) -> HirKind {}
    pub fn properties(&self) -> &Properties {}
    fn into_parts(mut self) -> (HirKind, Properties) {}
}
fn singleton_chars(hirs: &[Hir]) -> Option<Vec<char>> {
    let mut singletons = vec![];
    for hir in hirs.iter() {
        let literal = match *hir.kind() {
            HirKind::Literal(Literal(ref bytes)) => bytes,
            _ => return None,
        };
        let ch = match crate::debug::utf8_decode(literal) {
            None => return None,
            Some(Err(_)) => return None,
            Some(Ok(ch)) => ch,
        };
        if literal.len() != ch.len_utf8() {
            return None;
        }
        singletons.push(ch);
    }
    Some(singletons)
}
pub(crate) fn utf8_decode(bytes: &[u8]) -> Option<Result<char, u8>> {
    fn len(byte: u8) -> Option<usize> {
        if byte <= 0x7F {
            return Some(1);
        } else if byte & 0b1100_0000 == 0b1000_0000 {
            return None;
        } else if byte <= 0b1101_1111 {
            Some(2)
        } else if byte <= 0b1110_1111 {
            Some(3)
        } else if byte <= 0b1111_0111 {
            Some(4)
        } else {
            None
        }
    }
    if bytes.is_empty() {
        return None;
    }
    let len = match len(bytes[0]) {
        None => return Some(Err(bytes[0])),
        Some(len) if len > bytes.len() => return Some(Err(bytes[0])),
        Some(1) => return Some(Ok(char::from(bytes[0]))),
        Some(len) => len,
    };
    match core::str::from_utf8(&bytes[..len]) {
        Ok(s) => Some(Ok(s.chars().next().unwrap())),
        Err(_) => Some(Err(bytes[0])),
    }
}
