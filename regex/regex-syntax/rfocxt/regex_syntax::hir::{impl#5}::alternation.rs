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
pub struct ClassBytes {
    set: IntervalSet<ClassBytesRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
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
#[derive(Clone, Eq, PartialEq)]
pub enum Class {
    /// A set of characters represented by Unicode scalar values.
    Unicode(ClassUnicode),
    /// A set of characters represented by arbitrary bytes (one byte per
    /// character).
    Bytes(ClassBytes),
}
impl Hir {
    #[inline]
    pub fn empty() -> Hir {}
    #[inline]
    pub fn fail() -> Hir {
        let class = Class::Bytes(ClassBytes::empty());
        let props = Properties::class(&class);
        Hir {
            kind: HirKind::Class(class),
            props,
        }
    }
    #[inline]
    pub fn literal<B: Into<Box<[u8]>>>(lit: B) -> Hir {}
    #[inline]
    pub fn class(class: Class) -> Hir {
        if class.is_empty() {
            return Hir::fail();
        } else if let Some(bytes) = class.literal() {
            return Hir::literal(bytes);
        }
        let props = Properties::class(&class);
        Hir {
            kind: HirKind::Class(class),
            props,
        }
    }
    #[inline]
    pub fn look(look: Look) -> Hir {}
    #[inline]
    pub fn repetition(mut rep: Repetition) -> Hir {}
    #[inline]
    pub fn capture(capture: Capture) -> Hir {}
    pub fn concat(subs: Vec<Hir>) -> Hir {}
    pub fn alternation(subs: Vec<Hir>) -> Hir {
        let mut new = Vec::with_capacity(subs.len());
        for sub in subs {
            let (kind, props) = sub.into_parts();
            match kind {
                HirKind::Alternation(subs2) => {
                    new.extend(subs2);
                }
                kind => {
                    new.push(Hir { kind, props });
                }
            }
        }
        if new.is_empty() {
            return Hir::fail();
        } else if new.len() == 1 {
            return new.pop().unwrap();
        }
        if let Some(singletons) = singleton_chars(&new) {
            let it = singletons
                .into_iter()
                .map(|ch| ClassUnicodeRange {
                    start: ch,
                    end: ch,
                });
            return Hir::class(Class::Unicode(ClassUnicode::new(it)));
        }
        if let Some(singletons) = singleton_bytes(&new) {
            let it = singletons
                .into_iter()
                .map(|b| ClassBytesRange {
                    start: b,
                    end: b,
                });
            return Hir::class(Class::Bytes(ClassBytes::new(it)));
        }
        if let Some(cls) = class_chars(&new) {
            return Hir::class(cls);
        }
        if let Some(cls) = class_bytes(&new) {
            return Hir::class(cls);
        }
        new = match lift_common_prefix(new) {
            Ok(hir) => return hir,
            Err(unchanged) => unchanged,
        };
        let props = Properties::alternation(&new);
        Hir {
            kind: HirKind::Alternation(new),
            props,
        }
    }
    #[inline]
    pub fn dot(dot: Dot) -> Hir {}
}
impl Properties {
    fn empty() -> Properties {}
    fn literal(lit: &Literal) -> Properties {}
    fn class(class: &Class) -> Properties {}
    fn look(look: Look) -> Properties {}
    fn repetition(rep: &Repetition) -> Properties {}
    fn capture(capture: &Capture) -> Properties {}
    fn concat(concat: &[Hir]) -> Properties {}
    fn alternation(alts: &[Hir]) -> Properties {
        Properties::union(alts.iter().map(|hir| hir.properties()))
    }
}
fn singleton_bytes(hirs: &[Hir]) -> Option<Vec<u8>> {
    let mut singletons = vec![];
    for hir in hirs.iter() {
        let literal = match *hir.kind() {
            HirKind::Literal(Literal(ref bytes)) => bytes,
            _ => return None,
        };
        if literal.len() != 1 {
            return None;
        }
        singletons.push(literal[0]);
    }
    Some(singletons)
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
fn class_chars(hirs: &[Hir]) -> Option<Class> {
    let mut cls = ClassUnicode::new(vec![]);
    for hir in hirs.iter() {
        match *hir.kind() {
            HirKind::Class(Class::Unicode(ref cls2)) => {
                cls.union(cls2);
            }
            HirKind::Class(Class::Bytes(ref cls2)) => {
                cls.union(&cls2.to_unicode_class()?);
            }
            _ => return None,
        };
    }
    Some(Class::Unicode(cls))
}
fn class_bytes(hirs: &[Hir]) -> Option<Class> {
    let mut cls = ClassBytes::new(vec![]);
    for hir in hirs.iter() {
        match *hir.kind() {
            HirKind::Class(Class::Unicode(ref cls2)) => {
                cls.union(&cls2.to_byte_class()?);
            }
            HirKind::Class(Class::Bytes(ref cls2)) => {
                cls.union(cls2);
            }
            _ => return None,
        };
    }
    Some(Class::Bytes(cls))
}
