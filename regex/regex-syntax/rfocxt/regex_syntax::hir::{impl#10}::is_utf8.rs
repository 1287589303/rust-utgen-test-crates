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
pub struct ClassBytes {
    set: IntervalSet<ClassBytesRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ClassUnicode {
    /// The span of this class.
    pub span: Span,
    /// Whether this class is negated or not.
    ///
    /// Note: be careful when using this attribute. This specifically refers
    /// to whether the class is written as `\p` or `\P`, where the latter
    /// is `negated = true`. However, it also possible to write something like
    /// `\P{scx!=Katakana}` which is actually equivalent to
    /// `\p{scx=Katakana}` and is therefore not actually negated even though
    /// `negated = true` here. To test whether this class is truly negated
    /// or not, use the `is_negated` method.
    pub negated: bool,
    /// The kind of Unicode class.
    pub kind: ClassUnicodeKind,
}
#[derive(Clone, Eq, PartialEq)]
pub enum Class {
    /// A set of characters represented by Unicode scalar values.
    Unicode(ClassUnicode),
    /// A set of characters represented by arbitrary bytes (one byte per
    /// character).
    Bytes(ClassBytes),
}
impl Class {
    pub fn case_fold_simple(&mut self) {}
    pub fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError> {}
    pub fn negate(&mut self) {}
    pub fn is_utf8(&self) -> bool {
        match *self {
            Class::Unicode(_) => true,
            Class::Bytes(ref x) => x.is_ascii(),
        }
    }
    pub fn minimum_len(&self) -> Option<usize> {}
    pub fn maximum_len(&self) -> Option<usize> {}
    pub fn is_empty(&self) -> bool {}
    pub fn literal(&self) -> Option<Vec<u8>> {}
}
impl ClassBytes {
    pub fn new<I>(ranges: I) -> ClassBytes
    where
        I: IntoIterator<Item = ClassBytesRange>,
    {}
    pub fn empty() -> ClassBytes {}
    pub fn push(&mut self, range: ClassBytesRange) {}
    pub fn iter(&self) -> ClassBytesIter<'_> {}
    pub fn ranges(&self) -> &[ClassBytesRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassBytes) {}
    pub fn intersect(&mut self, other: &ClassBytes) {}
    pub fn difference(&mut self, other: &ClassBytes) {}
    pub fn symmetric_difference(&mut self, other: &ClassBytes) {}
    pub fn is_ascii(&self) -> bool {
        self.set.intervals().last().map_or(true, |r| r.end <= 0x7F)
    }
    pub fn minimum_len(&self) -> Option<usize> {}
    pub fn maximum_len(&self) -> Option<usize> {}
    pub fn literal(&self) -> Option<Vec<u8>> {}
    pub fn to_unicode_class(&self) -> Option<ClassUnicode> {}
}
