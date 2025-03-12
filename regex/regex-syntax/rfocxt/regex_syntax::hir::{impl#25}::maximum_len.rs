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
    #[inline]
    pub fn minimum_len(&self) -> Option<usize> {}
    #[inline]
    pub fn maximum_len(&self) -> Option<usize> {
        self.0.maximum_len
    }
    #[inline]
    pub fn look_set(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_suffix(&self) -> LookSet {}
    #[inline]
    pub fn look_set_suffix_any(&self) -> LookSet {}
    #[inline]
    pub fn is_utf8(&self) -> bool {}
    #[inline]
    pub fn explicit_captures_len(&self) -> usize {}
    #[inline]
    pub fn static_explicit_captures_len(&self) -> Option<usize> {}
    #[inline]
    pub fn is_literal(&self) -> bool {}
    #[inline]
    pub fn is_alternation_literal(&self) -> bool {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
    pub fn union<I, P>(props: I) -> Properties
    where
        I: IntoIterator<Item = P>,
        P: core::borrow::Borrow<Properties>,
    {}
}
