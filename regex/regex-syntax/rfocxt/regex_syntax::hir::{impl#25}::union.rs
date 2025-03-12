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
    pub fn minimum_len(&self) -> Option<usize> {
        self.0.minimum_len
    }
    #[inline]
    pub fn maximum_len(&self) -> Option<usize> {
        self.0.maximum_len
    }
    #[inline]
    pub fn look_set(&self) -> LookSet {
        self.0.look_set
    }
    #[inline]
    pub fn look_set_prefix(&self) -> LookSet {
        self.0.look_set_prefix
    }
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {
        self.0.look_set_prefix_any
    }
    #[inline]
    pub fn look_set_suffix(&self) -> LookSet {
        self.0.look_set_suffix
    }
    #[inline]
    pub fn look_set_suffix_any(&self) -> LookSet {
        self.0.look_set_suffix_any
    }
    #[inline]
    pub fn is_utf8(&self) -> bool {
        self.0.utf8
    }
    #[inline]
    pub fn explicit_captures_len(&self) -> usize {
        self.0.explicit_captures_len
    }
    #[inline]
    pub fn static_explicit_captures_len(&self) -> Option<usize> {
        self.0.static_explicit_captures_len
    }
    #[inline]
    pub fn is_literal(&self) -> bool {
        self.0.literal
    }
    #[inline]
    pub fn is_alternation_literal(&self) -> bool {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
    pub fn union<I, P>(props: I) -> Properties
    where
        I: IntoIterator<Item = P>,
        P: core::borrow::Borrow<Properties>,
    {
        let mut it = props.into_iter().peekable();
        let fix = if it.peek().is_none() { LookSet::empty() } else { LookSet::full() };
        let static_explicit_captures_len = it
            .peek()
            .and_then(|p| p.borrow().static_explicit_captures_len());
        let mut props = PropertiesI {
            minimum_len: None,
            maximum_len: None,
            look_set: LookSet::empty(),
            look_set_prefix: fix,
            look_set_suffix: fix,
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len,
            literal: false,
            alternation_literal: true,
        };
        let (mut min_poisoned, mut max_poisoned) = (false, false);
        for prop in it {
            let p = prop.borrow();
            props.look_set.set_union(p.look_set());
            props.look_set_prefix.set_intersect(p.look_set_prefix());
            props.look_set_suffix.set_intersect(p.look_set_suffix());
            props.look_set_prefix_any.set_union(p.look_set_prefix_any());
            props.look_set_suffix_any.set_union(p.look_set_suffix_any());
            props.utf8 = props.utf8 && p.is_utf8();
            props.explicit_captures_len = props
                .explicit_captures_len
                .saturating_add(p.explicit_captures_len());
            if props.static_explicit_captures_len != p.static_explicit_captures_len() {
                props.static_explicit_captures_len = None;
            }
            props.alternation_literal = props.alternation_literal && p.is_literal();
            if !min_poisoned {
                if let Some(xmin) = p.minimum_len() {
                    if props.minimum_len.map_or(true, |pmin| xmin < pmin) {
                        props.minimum_len = Some(xmin);
                    }
                } else {
                    props.minimum_len = None;
                    min_poisoned = true;
                }
            }
            if !max_poisoned {
                if let Some(xmax) = p.maximum_len() {
                    if props.maximum_len.map_or(true, |pmax| xmax > pmax) {
                        props.maximum_len = Some(xmax);
                    }
                } else {
                    props.maximum_len = None;
                    max_poisoned = true;
                }
            }
        }
        Properties(Box::new(props))
    }
}
impl LookSet {
    #[inline]
    pub fn empty() -> LookSet {
        LookSet { bits: 0 }
    }
    #[inline]
    pub fn full() -> LookSet {
        LookSet { bits: !0 }
    }
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
    pub fn set_union(&mut self, other: LookSet) {
        *self = self.union(other);
    }
    #[inline]
    pub fn intersect(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_intersect(&mut self, other: LookSet) {
        *self = self.intersect(other);
    }
    #[inline]
    pub fn read_repr(slice: &[u8]) -> LookSet {}
    #[inline]
    pub fn write_repr(self, slice: &mut [u8]) {}
}
