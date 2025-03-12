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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Properties(Box<PropertiesI>);
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct LookSet {
    /// The underlying representation this set is exposed to make it possible
    /// to store it somewhere efficiently. The representation is that
    /// of a bitset, where each assertion occupies bit `i` where `i =
    /// Look::as_repr()`.
    ///
    /// Note that users of this internal representation must permit the full
    /// range of `u16` values to be represented. For example, even if the
    /// current implementation only makes use of the 10 least significant bits,
    /// it may use more bits in a future semver compatible release.
    pub bits: u32,
}
#[derive(Clone, Eq, PartialEq)]
pub struct Literal(pub Box<[u8]>);
#[derive(Clone, Debug, Eq, PartialEq)]
struct PropertiesI {
    minimum_len: Option<usize>,
    maximum_len: Option<usize>,
    look_set: LookSet,
    look_set_prefix: LookSet,
    look_set_suffix: LookSet,
    look_set_prefix_any: LookSet,
    look_set_suffix_any: LookSet,
    utf8: bool,
    explicit_captures_len: usize,
    static_explicit_captures_len: Option<usize>,
    literal: bool,
    alternation_literal: bool,
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
impl LookSet {
    #[inline]
    pub fn empty() -> LookSet {
        LookSet { bits: 0 }
    }
    #[inline]
    pub fn full() -> LookSet {}
    #[inline]
    pub fn singleton(look: Look) -> LookSet {}
    #[inline]
    pub fn len(self) -> usize {}
    #[inline]
    pub fn is_empty(self) -> bool {}
    #[inline]
    pub fn contains(self, look: Look) -> bool {}
    #[inline]
    pub fn contains_anchor(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_haystack(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_line(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_lf(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_crlf(&self) -> bool {}
    #[inline]
    pub fn contains_word(self) -> bool {}
    #[inline]
    pub fn contains_word_unicode(self) -> bool {}
    #[inline]
    pub fn contains_word_ascii(self) -> bool {}
    #[inline]
    pub fn iter(self) -> LookSetIter {}
    #[inline]
    pub fn insert(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_insert(&mut self, look: Look) {}
    #[inline]
    pub fn remove(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_remove(&mut self, look: Look) {}
    #[inline]
    pub fn subtract(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_subtract(&mut self, other: LookSet) {}
    #[inline]
    pub fn union(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_union(&mut self, other: LookSet) {}
    #[inline]
    pub fn intersect(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_intersect(&mut self, other: LookSet) {}
    #[inline]
    pub fn read_repr(slice: &[u8]) -> LookSet {}
    #[inline]
    pub fn write_repr(self, slice: &mut [u8]) {}
}
