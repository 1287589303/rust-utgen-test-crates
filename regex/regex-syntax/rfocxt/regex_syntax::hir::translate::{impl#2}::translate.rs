type Result<T> = core::result::Result<T, Error>;
use core::cell::{Cell, RefCell};
use alloc::{boxed::Box, string::ToString, vec, vec::Vec};
use crate::{
    ast::{self, Ast, Span, Visitor},
    either::Either, hir::{self, Error, ErrorKind, Hir, HirKind},
    unicode::{self, ClassQuery},
};
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
#[derive(Clone, Debug)]
struct TranslatorI<'t, 'p> {
    trans: &'t Translator,
    pattern: &'p str,
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
#[derive(Clone, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    props: Properties,
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
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Flags {
    /// The span of this group of flags.
    pub span: Span,
    /// A sequence of flag items. Each item is either a flag or a negation
    /// operator.
    pub items: Vec<FlagsItem>,
}
#[derive(Clone, Debug)]
enum HirFrame {
    /// An arbitrary HIR expression. These get pushed whenever we hit a base
    /// case in the Ast. They get popped after an inductive (i.e., recursive)
    /// step is complete.
    Expr(Hir),
    /// A literal that is being constructed, character by character, from the
    /// AST. We need this because the AST gives each individual character its
    /// own node. So as we see characters, we peek at the top-most HirFrame.
    /// If it's a literal, then we add to it. Otherwise, we push a new literal.
    /// When it comes time to pop it, we convert it to an Hir via Hir::literal.
    Literal(Vec<u8>),
    /// A Unicode character class. This frame is mutated as we descend into
    /// the Ast of a character class (which is itself its own mini recursive
    /// structure).
    ClassUnicode(hir::ClassUnicode),
    /// A byte-oriented character class. This frame is mutated as we descend
    /// into the Ast of a character class (which is itself its own mini
    /// recursive structure).
    ///
    /// Byte character classes are created when Unicode mode (`u`) is disabled.
    /// If `utf8` is enabled (the default), then a byte character is only
    /// permitted to match ASCII text.
    ClassBytes(hir::ClassBytes),
    /// This is pushed whenever a repetition is observed. After visiting every
    /// sub-expression in the repetition, the translator's stack is expected to
    /// have this sentinel at the top.
    ///
    /// This sentinel only exists to stop other things (like flattening
    /// literals) from reaching across repetition operators.
    Repetition,
    /// This is pushed on to the stack upon first seeing any kind of capture,
    /// indicated by parentheses (including non-capturing groups). It is popped
    /// upon leaving a group.
    Group {
        /// The old active flags when this group was opened.
        ///
        /// If this group sets flags, then the new active flags are set to the
        /// result of merging the old flags with the flags introduced by this
        /// group. If the group doesn't set any flags, then this is simply
        /// equivalent to whatever flags were set when the group was opened.
        ///
        /// When this group is popped, the active flags should be restored to
        /// the flags set here.
        ///
        /// The "active" flags correspond to whatever flags are set in the
        /// Translator.
        old_flags: Flags,
    },
    /// This is pushed whenever a concatenation is observed. After visiting
    /// every sub-expression in the concatenation, the translator's stack is
    /// popped until it sees a Concat frame.
    Concat,
    /// This is pushed whenever an alternation is observed. After visiting
    /// every sub-expression in the alternation, the translator's stack is
    /// popped until it sees an Alternation frame.
    Alternation,
    /// This is pushed immediately before each sub-expression in an
    /// alternation. This separates the branches of an alternation on the
    /// stack and prevents literal flattening from reaching across alternation
    /// branches.
    ///
    /// It is popped after each expression in a branch until an 'Alternation'
    /// frame is observed when doing a post visit on an alternation.
    AlternationBranch,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Ast {
    /// An empty regex that matches everything.
    Empty(Box<Span>),
    /// A set of flags, e.g., `(?is)`.
    Flags(Box<SetFlags>),
    /// A single character literal, which includes escape sequences.
    Literal(Box<Literal>),
    /// The "any character" class.
    Dot(Box<Span>),
    /// A single zero-width assertion.
    Assertion(Box<Assertion>),
    /// A single Unicode character class, e.g., `\pL` or `\p{Greek}`.
    ClassUnicode(Box<ClassUnicode>),
    /// A single perl character class, e.g., `\d` or `\W`.
    ClassPerl(Box<ClassPerl>),
    /// A single bracketed character class set, which may contain zero or more
    /// character ranges and/or zero or more nested classes. e.g.,
    /// `[a-zA-Z\pL]`.
    ClassBracketed(Box<ClassBracketed>),
    /// A repetition operator applied to an arbitrary regular expression.
    Repetition(Box<Repetition>),
    /// A grouped regular expression.
    Group(Box<Group>),
    /// An alternation of regular expressions.
    Alternation(Box<Alternation>),
    /// A concatenation of regular expressions.
    Concat(Box<Concat>),
}
impl Translator {
    pub fn new() -> Translator {}
    pub fn translate(&mut self, pattern: &str, ast: &Ast) -> Result<Hir> {
        ast::visit(ast, TranslatorI::new(self, pattern))
    }
}
impl<'t, 'p> TranslatorI<'t, 'p> {
    fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {
        TranslatorI { trans, pattern }
    }
    fn trans(&self) -> &Translator {}
    fn push(&self, frame: HirFrame) {}
    fn push_char(&self, ch: char) {}
    fn push_byte(&self, byte: u8) {}
    fn pop(&self) -> Option<HirFrame> {}
    fn pop_concat_expr(&self) -> Option<Hir> {}
    fn pop_alt_expr(&self) -> Option<Hir> {}
    fn error(&self, span: Span, kind: ErrorKind) -> Error {}
    fn flags(&self) -> Flags {}
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
