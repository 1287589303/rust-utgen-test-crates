type Result<T> = core::result::Result<T, Error>;
use core::cell::{Cell, RefCell};
use alloc::{boxed::Box, string::ToString, vec, vec::Vec};
use crate::{
    ast::{self, Ast, Span, Visitor},
    either::Either, hir::{self, Error, ErrorKind, Hir, HirKind},
    unicode::{self, ClassQuery},
};
pub trait Visitor {
    type Output;
    type Err;
    fn finish(self) -> Result<Self::Output, Self::Err>;
    fn start(&mut self);
    fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_item_pre(
        &mut self,
        _ast: &ast::ClassSetItem,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_item_post(
        &mut self,
        _ast: &ast::ClassSetItem,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_binary_op_pre(
        &mut self,
        _ast: &ast::ClassSetBinaryOp,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_binary_op_post(
        &mut self,
        _ast: &ast::ClassSetBinaryOp,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_binary_op_in(
        &mut self,
        _ast: &ast::ClassSetBinaryOp,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
}
#[derive(Clone, Debug)]
struct TranslatorI<'t, 'p> {
    trans: &'t Translator,
    pattern: &'p str,
}
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
    crlf: Option<bool>,
}
#[derive(Clone, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    props: Properties,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the translator's Ast was parsed from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error, derived from the Ast given to the translator.
    span: Span,
}
#[derive(Clone, Debug)]
pub struct Translator {
    /// Our call stack, but on the heap.
    stack: RefCell<Vec<HirFrame>>,
    /// The current flag settings.
    flags: Cell<Flags>,
    /// Whether we're allowed to produce HIR that can match arbitrary bytes.
    utf8: bool,
    /// The line terminator to use for `.`.
    line_terminator: u8,
}
#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
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
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// This error occurs when a Unicode feature is used when Unicode
    /// support is disabled. For example `(?-u:\pL)` would trigger this error.
    UnicodeNotAllowed,
    /// This error occurs when translating a pattern that could match a byte
    /// sequence that isn't UTF-8 and `utf8` was enabled.
    InvalidUtf8,
    /// This error occurs when one uses a non-ASCII byte for a line terminator,
    /// but where Unicode mode is enabled and UTF-8 mode is disabled.
    InvalidLineTerminator,
    /// This occurs when an unrecognized Unicode property name could not
    /// be found.
    UnicodePropertyNotFound,
    /// This occurs when an unrecognized Unicode property value could not
    /// be found.
    UnicodePropertyValueNotFound,
    /// This occurs when a Unicode-aware Perl character class (`\w`, `\s` or
    /// `\d`) could not be found. This can occur when the `unicode-perl`
    /// crate feature is not enabled.
    UnicodePerlClassNotFound,
    /// This occurs when the Unicode simple case mapping tables are not
    /// available, and the regular expression required Unicode aware case
    /// insensitivity.
    UnicodeCaseUnavailable,
}
impl<'t, 'p> TranslatorI<'t, 'p> {
    fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {}
    fn trans(&self) -> &Translator {
        &self.trans
    }
    fn push(&self, frame: HirFrame) {}
    fn push_char(&self, ch: char) {}
    fn push_byte(&self, byte: u8) {}
    fn pop(&self) -> Option<HirFrame> {}
    fn pop_concat_expr(&self) -> Option<Hir> {}
    fn pop_alt_expr(&self) -> Option<Hir> {}
    fn error(&self, span: Span, kind: ErrorKind) -> Error {
        Error {
            kind,
            pattern: self.pattern.to_string(),
            span,
        }
    }
    fn flags(&self) -> Flags {
        self.trans().flags.get()
    }
    fn set_flags(&self, ast_flags: &ast::Flags) -> Flags {}
    fn ast_literal_to_scalar(&self, lit: &ast::Literal) -> Result<Either<char, u8>> {}
    fn case_fold_char(&self, span: Span, c: char) -> Result<Option<Hir>> {}
    fn hir_dot(&self, span: Span) -> Result<Hir> {
        let (utf8, lineterm, flags) = (
            self.trans().utf8,
            self.trans().line_terminator,
            self.flags(),
        );
        if utf8 && (!flags.unicode() || !lineterm.is_ascii()) {
            return Err(self.error(span, ErrorKind::InvalidUtf8));
        }
        let dot = if flags.dot_matches_new_line() {
            if flags.unicode() { hir::Dot::AnyChar } else { hir::Dot::AnyByte }
        } else {
            if flags.unicode() {
                if flags.crlf() {
                    hir::Dot::AnyCharExceptCRLF
                } else {
                    if !lineterm.is_ascii() {
                        return Err(self.error(span, ErrorKind::InvalidLineTerminator));
                    }
                    hir::Dot::AnyCharExcept(char::from(lineterm))
                }
            } else {
                if flags.crlf() {
                    hir::Dot::AnyByteExceptCRLF
                } else {
                    hir::Dot::AnyByteExcept(lineterm)
                }
            }
        };
        Ok(Hir::dot(dot))
    }
    fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {}
    fn hir_capture(&self, group: &ast::Group, expr: Hir) -> Hir {}
    fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {}
    fn hir_unicode_class(
        &self,
        ast_class: &ast::ClassUnicode,
    ) -> Result<hir::ClassUnicode> {}
    fn hir_ascii_unicode_class(
        &self,
        ast: &ast::ClassAscii,
    ) -> Result<hir::ClassUnicode> {}
    fn hir_ascii_byte_class(&self, ast: &ast::ClassAscii) -> Result<hir::ClassBytes> {}
    fn hir_perl_unicode_class(
        &self,
        ast_class: &ast::ClassPerl,
    ) -> Result<hir::ClassUnicode> {}
    fn hir_perl_byte_class(
        &self,
        ast_class: &ast::ClassPerl,
    ) -> Result<hir::ClassBytes> {}
    fn convert_unicode_class_error(
        &self,
        span: &Span,
        result: core::result::Result<hir::ClassUnicode, unicode::Error>,
    ) -> Result<hir::ClassUnicode> {}
    fn unicode_fold_and_negate(
        &self,
        span: &Span,
        negated: bool,
        class: &mut hir::ClassUnicode,
    ) -> Result<()> {}
    fn bytes_fold_and_negate(
        &self,
        span: &Span,
        negated: bool,
        class: &mut hir::ClassBytes,
    ) -> Result<()> {}
    fn class_literal_byte(&self, ast: &ast::Literal) -> Result<u8> {}
}
impl Flags {
    fn from_ast(ast: &ast::Flags) -> Flags {}
    fn merge(&mut self, previous: &Flags) {}
    fn case_insensitive(&self) -> bool {}
    fn multi_line(&self) -> bool {}
    fn dot_matches_new_line(&self) -> bool {
        self.dot_matches_new_line.unwrap_or(false)
    }
    fn swap_greed(&self) -> bool {}
    fn unicode(&self) -> bool {
        self.unicode.unwrap_or(true)
    }
    fn crlf(&self) -> bool {
        self.crlf.unwrap_or(false)
    }
}
impl Hir {
    #[inline]
    pub fn empty() -> Hir {}
    #[inline]
    pub fn fail() -> Hir {}
    #[inline]
    pub fn literal<B: Into<Box<[u8]>>>(lit: B) -> Hir {}
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
