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
pub struct ClassBytes {
    set: IntervalSet<ClassBytesRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassBytesRange {
    start: u8,
    end: u8,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassUnicodeRange {
    start: char,
    end: char,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Properties(Box<PropertiesI>);
#[derive(Clone, Eq, PartialEq)]
pub enum Class {
    /// A set of characters represented by Unicode scalar values.
    Unicode(ClassUnicode),
    /// A set of characters represented by arbitrary bytes (one byte per
    /// character).
    Bytes(ClassBytes),
}
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dot {
    /// Matches the UTF-8 encoding of any Unicode scalar value.
    ///
    /// This is equivalent to `(?su:.)` and also `\p{any}`.
    AnyChar,
    /// Matches any byte value.
    ///
    /// This is equivalent to `(?s-u:.)` and also `(?-u:[\x00-\xFF])`.
    AnyByte,
    /// Matches the UTF-8 encoding of any Unicode scalar value except for the
    /// `char` given.
    ///
    /// This is equivalent to using `(?u-s:.)` with the line terminator set
    /// to a particular ASCII byte. (Because of peculiarities in the regex
    /// engines, a line terminator must be a single byte. It follows that when
    /// UTF-8 mode is enabled, this single byte must also be a Unicode scalar
    /// value. That is, ti must be ASCII.)
    ///
    /// (This and `AnyCharExceptLF` both exist because of legacy reasons.
    /// `AnyCharExceptLF` will be dropped in the next breaking change release.)
    AnyCharExcept(char),
    /// Matches the UTF-8 encoding of any Unicode scalar value except for `\n`.
    ///
    /// This is equivalent to `(?u-s:.)` and also `[\p{any}--\n]`.
    AnyCharExceptLF,
    /// Matches the UTF-8 encoding of any Unicode scalar value except for `\r`
    /// and `\n`.
    ///
    /// This is equivalent to `(?uR-s:.)` and also `[\p{any}--\r\n]`.
    AnyCharExceptCRLF,
    /// Matches any byte value except for the `u8` given.
    ///
    /// This is equivalent to using `(?-us:.)` with the line terminator set
    /// to a particular ASCII byte. (Because of peculiarities in the regex
    /// engines, a line terminator must be a single byte. It follows that when
    /// UTF-8 mode is enabled, this single byte must also be a Unicode scalar
    /// value. That is, ti must be ASCII.)
    ///
    /// (This and `AnyByteExceptLF` both exist because of legacy reasons.
    /// `AnyByteExceptLF` will be dropped in the next breaking change release.)
    AnyByteExcept(u8),
    /// Matches any byte value except for `\n`.
    ///
    /// This is equivalent to `(?-su:.)` and also `(?-u:[[\x00-\xFF]--\n])`.
    AnyByteExceptLF,
    /// Matches any byte value except for `\r` and `\n`.
    ///
    /// This is equivalent to `(?R-su:.)` and also `(?-u:[[\x00-\xFF]--\r\n])`.
    AnyByteExceptCRLF,
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
    pub fn empty() -> Hir {}
    #[inline]
    pub fn fail() -> Hir {}
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
    pub fn alternation(subs: Vec<Hir>) -> Hir {}
    #[inline]
    pub fn dot(dot: Dot) -> Hir {
        match dot {
            Dot::AnyChar => {
                Hir::class(
                    Class::Unicode(
                        ClassUnicode::new([ClassUnicodeRange::new('\0', '\u{10FFFF}')]),
                    ),
                )
            }
            Dot::AnyByte => {
                Hir::class(
                    Class::Bytes(ClassBytes::new([ClassBytesRange::new(b'\0', b'\xFF')])),
                )
            }
            Dot::AnyCharExcept(ch) => {
                let mut cls = ClassUnicode::new([ClassUnicodeRange::new(ch, ch)]);
                cls.negate();
                Hir::class(Class::Unicode(cls))
            }
            Dot::AnyCharExceptLF => {
                Hir::class(
                    Class::Unicode(
                        ClassUnicode::new([
                            ClassUnicodeRange::new('\0', '\x09'),
                            ClassUnicodeRange::new('\x0B', '\u{10FFFF}'),
                        ]),
                    ),
                )
            }
            Dot::AnyCharExceptCRLF => {
                Hir::class(
                    Class::Unicode(
                        ClassUnicode::new([
                            ClassUnicodeRange::new('\0', '\x09'),
                            ClassUnicodeRange::new('\x0B', '\x0C'),
                            ClassUnicodeRange::new('\x0E', '\u{10FFFF}'),
                        ]),
                    ),
                )
            }
            Dot::AnyByteExcept(byte) => {
                let mut cls = ClassBytes::new([ClassBytesRange::new(byte, byte)]);
                cls.negate();
                Hir::class(Class::Bytes(cls))
            }
            Dot::AnyByteExceptLF => {
                Hir::class(
                    Class::Bytes(
                        ClassBytes::new([
                            ClassBytesRange::new(b'\0', b'\x09'),
                            ClassBytesRange::new(b'\x0B', b'\xFF'),
                        ]),
                    ),
                )
            }
            Dot::AnyByteExceptCRLF => {
                Hir::class(
                    Class::Bytes(
                        ClassBytes::new([
                            ClassBytesRange::new(b'\0', b'\x09'),
                            ClassBytesRange::new(b'\x0B', b'\x0C'),
                            ClassBytesRange::new(b'\x0E', b'\xFF'),
                        ]),
                    ),
                )
            }
        }
    }
}
impl ClassBytes {
    pub fn new<I>(ranges: I) -> ClassBytes
    where
        I: IntoIterator<Item = ClassBytesRange>,
    {
        ClassBytes {
            set: IntervalSet::new(ranges),
        }
    }
    pub fn empty() -> ClassBytes {}
    pub fn push(&mut self, range: ClassBytesRange) {}
    pub fn iter(&self) -> ClassBytesIter<'_> {}
    pub fn ranges(&self) -> &[ClassBytesRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {
        self.set.negate();
    }
    pub fn union(&mut self, other: &ClassBytes) {}
    pub fn intersect(&mut self, other: &ClassBytes) {}
    pub fn difference(&mut self, other: &ClassBytes) {}
    pub fn symmetric_difference(&mut self, other: &ClassBytes) {}
    pub fn is_ascii(&self) -> bool {}
    pub fn minimum_len(&self) -> Option<usize> {}
    pub fn maximum_len(&self) -> Option<usize> {}
    pub fn literal(&self) -> Option<Vec<u8>> {}
    pub fn to_unicode_class(&self) -> Option<ClassUnicode> {}
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {
        ClassUnicode {
            set: IntervalSet::new(ranges),
        }
    }
    pub fn empty() -> ClassUnicode {}
    pub fn push(&mut self, range: ClassUnicodeRange) {}
    pub fn iter(&self) -> ClassUnicodeIter<'_> {}
    pub fn ranges(&self) -> &[ClassUnicodeRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError> {}
    pub fn negate(&mut self) {
        self.set.negate();
    }
    pub fn union(&mut self, other: &ClassUnicode) {}
    pub fn intersect(&mut self, other: &ClassUnicode) {}
    pub fn difference(&mut self, other: &ClassUnicode) {}
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {}
    pub fn is_ascii(&self) -> bool {}
    pub fn minimum_len(&self) -> Option<usize> {}
    pub fn maximum_len(&self) -> Option<usize> {}
    pub fn literal(&self) -> Option<Vec<u8>> {}
    pub fn to_byte_class(&self) -> Option<ClassBytes> {}
}
impl ClassBytesRange {
    pub fn new(start: u8, end: u8) -> ClassBytesRange {
        ClassBytesRange::create(start, end)
    }
    pub fn start(&self) -> u8 {}
    pub fn end(&self) -> u8 {}
    pub fn len(&self) -> usize {}
}
impl ClassUnicodeRange {
    pub fn new(start: char, end: char) -> ClassUnicodeRange {
        ClassUnicodeRange::create(start, end)
    }
    pub fn start(&self) -> char {}
    pub fn end(&self) -> char {}
    pub fn len(&self) -> usize {}
}
