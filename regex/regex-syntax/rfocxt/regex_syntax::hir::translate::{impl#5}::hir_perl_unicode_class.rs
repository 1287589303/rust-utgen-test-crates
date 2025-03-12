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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ClassPerl {
    /// The span of this class.
    pub span: Span,
    /// The kind of Perl class.
    pub kind: ClassPerlKind,
    /// Whether the class is negated or not. e.g., `\d` is not negated but
    /// `\D` is.
    pub negated: bool,
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
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
    #[allow(dead_code)]
    PerlClassNotFound,
}
impl<'t, 'p> TranslatorI<'t, 'p> {
    fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {}
    fn trans(&self) -> &Translator {}
    fn push(&self, frame: HirFrame) {}
    fn push_char(&self, ch: char) {}
    fn push_byte(&self, byte: u8) {}
    fn pop(&self) -> Option<HirFrame> {}
    fn pop_concat_expr(&self) -> Option<Hir> {}
    fn pop_alt_expr(&self) -> Option<Hir> {}
    fn error(&self, span: Span, kind: ErrorKind) -> Error {}
    fn flags(&self) -> Flags {
        self.trans().flags.get()
    }
    fn set_flags(&self, ast_flags: &ast::Flags) -> Flags {}
    fn ast_literal_to_scalar(&self, lit: &ast::Literal) -> Result<Either<char, u8>> {}
    fn case_fold_char(&self, span: Span, c: char) -> Result<Option<Hir>> {}
    fn hir_dot(&self, span: Span) -> Result<Hir> {}
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
    ) -> Result<hir::ClassUnicode> {
        use crate::ast::ClassPerlKind::*;
        assert!(self.flags().unicode());
        let result = match ast_class.kind {
            Digit => unicode::perl_digit(),
            Space => unicode::perl_space(),
            Word => unicode::perl_word(),
        };
        let mut class = self.convert_unicode_class_error(&ast_class.span, result)?;
        if ast_class.negated {
            class.negate();
        }
        Ok(class)
    }
    fn hir_perl_byte_class(
        &self,
        ast_class: &ast::ClassPerl,
    ) -> Result<hir::ClassBytes> {}
    fn convert_unicode_class_error(
        &self,
        span: &Span,
        result: core::result::Result<hir::ClassUnicode, unicode::Error>,
    ) -> Result<hir::ClassUnicode> {
        result
            .map_err(|err| {
                let sp = span.clone();
                match err {
                    unicode::Error::PropertyNotFound => {
                        self.error(sp, ErrorKind::UnicodePropertyNotFound)
                    }
                    unicode::Error::PropertyValueNotFound => {
                        self.error(sp, ErrorKind::UnicodePropertyValueNotFound)
                    }
                    unicode::Error::PerlClassNotFound => {
                        self.error(sp, ErrorKind::UnicodePerlClassNotFound)
                    }
                }
            })
    }
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
    fn dot_matches_new_line(&self) -> bool {}
    fn swap_greed(&self) -> bool {}
    fn unicode(&self) -> bool {
        self.unicode.unwrap_or(true)
    }
    fn crlf(&self) -> bool {}
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {}
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
pub fn perl_space() -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(any(feature = "unicode-perl", feature = "unicode-bool")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        Err(Error::PerlClassNotFound)
    }
    #[cfg(all(feature = "unicode-perl", not(feature = "unicode-bool")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::perl_space::WHITE_SPACE;
        Ok(hir_class(WHITE_SPACE))
    }
    #[cfg(feature = "unicode-bool")]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::property_bool::WHITE_SPACE;
        Ok(hir_class(WHITE_SPACE))
    }
    imp()
}
pub fn perl_word() -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-perl"))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        Err(Error::PerlClassNotFound)
    }
    #[cfg(feature = "unicode-perl")]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::perl_word::PERL_WORD;
        Ok(hir_class(PERL_WORD))
    }
    imp()
}
pub fn perl_digit() -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(any(feature = "unicode-perl", feature = "unicode-gencat")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        Err(Error::PerlClassNotFound)
    }
    #[cfg(all(feature = "unicode-perl", not(feature = "unicode-gencat")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::perl_decimal::DECIMAL_NUMBER;
        Ok(hir_class(DECIMAL_NUMBER))
    }
    #[cfg(feature = "unicode-gencat")]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::general_category::DECIMAL_NUMBER;
        Ok(hir_class(DECIMAL_NUMBER))
    }
    imp()
}
