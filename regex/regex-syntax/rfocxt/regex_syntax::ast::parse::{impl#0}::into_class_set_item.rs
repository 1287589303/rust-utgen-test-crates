type Result<T> = core::result::Result<T, ast::Error>;
use core::{
    borrow::Borrow, cell::{Cell, RefCell},
    mem,
};
use alloc::{
    boxed::Box, string::{String, ToString},
    vec, vec::Vec,
};
use crate::{
    ast::{self, Ast, Position, Span},
    either::Either, is_escapeable_character, is_meta_character,
};
#[derive(Clone, Debug)]
struct ParserI<'s, P> {
    /// The parser state/configuration.
    parser: P,
    /// The full regular expression provided by the user.
    pattern: &'s str,
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
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the parser generated the error from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error.
    span: Span,
}
#[derive(Clone, Eq, PartialEq)]
pub struct Literal(pub Box<[u8]>);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Assertion {
    /// The span of this assertion.
    pub span: Span,
    /// The assertion kind, e.g., `\b` or `^`.
    pub kind: AssertionKind,
}
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Literal {
    bytes: Vec<u8>,
    exact: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Literal {
    /// The span of this literal.
    pub span: Span,
    /// The kind of this literal.
    pub kind: LiteralKind,
    /// The Unicode scalar value corresponding to this literal.
    pub c: char,
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
#[derive(Clone, Debug, Eq, PartialEq)]
enum Primitive {
    Literal(ast::Literal),
    Assertion(ast::Assertion),
    Dot(Span),
    Perl(ast::ClassPerl),
    Unicode(ast::ClassUnicode),
}
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum ErrorKind {
    /// The capturing group limit was exceeded.
    ///
    /// Note that this represents a limit on the total number of capturing
    /// groups in a regex and not necessarily the number of nested capturing
    /// groups. That is, the nest limit can be low and it is still possible for
    /// this error to occur.
    CaptureLimitExceeded,
    /// An invalid escape sequence was found in a character class set.
    ClassEscapeInvalid,
    /// An invalid character class range was found. An invalid range is any
    /// range where the start is greater than the end.
    ClassRangeInvalid,
    /// An invalid range boundary was found in a character class. Range
    /// boundaries must be a single literal codepoint, but this error indicates
    /// that something else was found, such as a nested class.
    ClassRangeLiteral,
    /// An opening `[` was found with no corresponding closing `]`.
    ClassUnclosed,
    /// Note that this error variant is no longer used. Namely, a decimal
    /// number can only appear as a repetition quantifier. When the number
    /// in a repetition quantifier is empty, then it gets its own specialized
    /// error, `RepetitionCountDecimalEmpty`.
    DecimalEmpty,
    /// An invalid decimal number was given where one was expected.
    DecimalInvalid,
    /// A bracketed hex literal was empty.
    EscapeHexEmpty,
    /// A bracketed hex literal did not correspond to a Unicode scalar value.
    EscapeHexInvalid,
    /// An invalid hexadecimal digit was found.
    EscapeHexInvalidDigit,
    /// EOF was found before an escape sequence was completed.
    EscapeUnexpectedEof,
    /// An unrecognized escape sequence.
    EscapeUnrecognized,
    /// A dangling negation was used when setting flags, e.g., `i-`.
    FlagDanglingNegation,
    /// A flag was used twice, e.g., `i-i`.
    FlagDuplicate {
        /// The position of the original flag. The error position
        /// points to the duplicate flag.
        original: Span,
    },
    /// The negation operator was used twice, e.g., `-i-s`.
    FlagRepeatedNegation {
        /// The position of the original negation operator. The error position
        /// points to the duplicate negation operator.
        original: Span,
    },
    /// Expected a flag but got EOF, e.g., `(?`.
    FlagUnexpectedEof,
    /// Unrecognized flag, e.g., `a`.
    FlagUnrecognized,
    /// A duplicate capture name was found.
    GroupNameDuplicate {
        /// The position of the initial occurrence of the capture name. The
        /// error position itself points to the duplicate occurrence.
        original: Span,
    },
    /// A capture group name is empty, e.g., `(?P<>abc)`.
    GroupNameEmpty,
    /// An invalid character was seen for a capture group name. This includes
    /// errors where the first character is a digit (even though subsequent
    /// characters are allowed to be digits).
    GroupNameInvalid,
    /// A closing `>` could not be found for a capture group name.
    GroupNameUnexpectedEof,
    /// An unclosed group, e.g., `(ab`.
    ///
    /// The span of this error corresponds to the unclosed parenthesis.
    GroupUnclosed,
    /// An unopened group, e.g., `ab)`.
    GroupUnopened,
    /// The nest limit was exceeded. The limit stored here is the limit
    /// configured in the parser.
    NestLimitExceeded(u32),
    /// The range provided in a counted repetition operator is invalid. The
    /// range is invalid if the start is greater than the end.
    RepetitionCountInvalid,
    /// An opening `{` was not followed by a valid decimal value.
    /// For example, `x{}` or `x{]}` would fail.
    RepetitionCountDecimalEmpty,
    /// An opening `{` was found with no corresponding closing `}`.
    RepetitionCountUnclosed,
    /// A repetition operator was applied to a missing sub-expression. This
    /// occurs, for example, in the regex consisting of just a `*` or even
    /// `(?i)*`. It is, however, possible to create a repetition operating on
    /// an empty sub-expression. For example, `()*` is still considered valid.
    RepetitionMissing,
    /// The special word boundary syntax, `\b{something}`, was used, but
    /// either EOF without `}` was seen, or an invalid character in the
    /// braces was seen.
    SpecialWordBoundaryUnclosed,
    /// The special word boundary syntax, `\b{something}`, was used, but
    /// `something` was not recognized as a valid word boundary kind.
    SpecialWordBoundaryUnrecognized,
    /// The syntax `\b{` was observed, but afterwards the end of the pattern
    /// was observed without being able to tell whether it was meant to be a
    /// bounded repetition on the `\b` or the beginning of a special word
    /// boundary assertion.
    SpecialWordOrRepetitionUnexpectedEof,
    /// The Unicode class is not valid. This typically occurs when a `\p` is
    /// followed by something other than a `{`.
    UnicodeClassInvalid,
    /// When octal support is disabled, this error is produced when an octal
    /// escape is used. The octal escape is assumed to be an invocation of
    /// a backreference, which is the common case.
    UnsupportedBackreference,
    /// When syntax similar to PCRE's look-around is used, this error is
    /// returned. Some example syntaxes that are rejected include, but are
    /// not necessarily limited to, `(?=re)`, `(?!re)`, `(?<=re)` and
    /// `(?<!re)`. Note that all of these syntaxes are otherwise invalid; this
    /// error is used to improve the user experience.
    UnsupportedLookAround,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum ClassSetItem {
    /// An empty item.
    ///
    /// Note that a bracketed character class cannot contain a single empty
    /// item. Empty items can appear when using one of the binary operators.
    /// For example, `[&&]` is the intersection of two empty classes.
    Empty(Span),
    /// A single literal.
    Literal(Literal),
    /// A range between two literals.
    Range(ClassSetRange),
    /// An ASCII character class, e.g., `[:alnum:]` or `[:punct:]`.
    Ascii(ClassAscii),
    /// A Unicode character class, e.g., `\pL` or `\p{Greek}`.
    Unicode(ClassUnicode),
    /// A perl character class, e.g., `\d` or `\W`.
    Perl(ClassPerl),
    /// A bracketed character class set, which may contain zero or more
    /// character ranges and/or zero or more nested classes. e.g.,
    /// `[a-zA-Z\pL]`.
    Bracketed(Box<ClassBracketed>),
    /// A union of items.
    Union(ClassSetUnion),
}
impl Primitive {
    fn span(&self) -> &Span {
        match *self {
            Primitive::Literal(ref x) => &x.span,
            Primitive::Assertion(ref x) => &x.span,
            Primitive::Dot(ref span) => span,
            Primitive::Perl(ref x) => &x.span,
            Primitive::Unicode(ref x) => &x.span,
        }
    }
    fn into_ast(self) -> Ast {}
    fn into_class_set_item<P: Borrow<Parser>>(
        self,
        p: &ParserI<'_, P>,
    ) -> Result<ast::ClassSetItem> {
        use self::Primitive::*;
        use crate::ast::ClassSetItem;
        match self {
            Literal(lit) => Ok(ClassSetItem::Literal(lit)),
            Perl(cls) => Ok(ClassSetItem::Perl(cls)),
            Unicode(cls) => Ok(ClassSetItem::Unicode(cls)),
            x => Err(p.error(*x.span(), ast::ErrorKind::ClassEscapeInvalid)),
        }
    }
    fn into_class_literal<P: Borrow<Parser>>(
        self,
        p: &ParserI<'_, P>,
    ) -> Result<ast::Literal> {}
}
impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
    fn new(parser: P, pattern: &'s str) -> ParserI<'s, P> {}
    fn parser(&self) -> &Parser {}
    fn pattern(&self) -> &str {}
    fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
        ast::Error {
            kind,
            pattern: self.pattern().to_string(),
            span,
        }
    }
    fn offset(&self) -> usize {}
    fn line(&self) -> usize {}
    fn column(&self) -> usize {}
    fn next_capture_index(&self, span: Span) -> Result<u32> {}
    fn add_capture_name(&self, cap: &ast::CaptureName) -> Result<()> {}
    fn ignore_whitespace(&self) -> bool {}
    fn char(&self) -> char {
        self.char_at(self.offset())
    }
    fn char_at(&self, i: usize) -> char {}
    fn bump(&self) -> bool {}
    fn bump_if(&self, prefix: &str) -> bool {}
    fn is_lookaround_prefix(&self) -> bool {}
    fn bump_and_bump_space(&self) -> bool {}
    fn bump_space(&self) {}
    fn peek(&self) -> Option<char> {}
    fn peek_space(&self) -> Option<char> {}
    fn is_eof(&self) -> bool {}
    fn pos(&self) -> Position {}
    fn span(&self) -> Span {}
    fn span_char(&self) -> Span {}
    #[inline(never)]
    fn push_alternate(&self, mut concat: ast::Concat) -> Result<ast::Concat> {}
    fn push_or_add_alternation(&self, concat: ast::Concat) {}
    #[inline(never)]
    fn push_group(&self, mut concat: ast::Concat) -> Result<ast::Concat> {}
    #[inline(never)]
    fn pop_group(&self, mut group_concat: ast::Concat) -> Result<ast::Concat> {}
    #[inline(never)]
    fn pop_group_end(&self, mut concat: ast::Concat) -> Result<Ast> {}
    #[inline(never)]
    fn push_class_open(
        &self,
        parent_union: ast::ClassSetUnion,
    ) -> Result<ast::ClassSetUnion> {}
    #[inline(never)]
    fn pop_class(
        &self,
        nested_union: ast::ClassSetUnion,
    ) -> Result<Either<ast::ClassSetUnion, ast::ClassBracketed>> {}
    #[inline(never)]
    fn unclosed_class_error(&self) -> ast::Error {}
    #[inline(never)]
    fn push_class_op(
        &self,
        next_kind: ast::ClassSetBinaryOpKind,
        next_union: ast::ClassSetUnion,
    ) -> ast::ClassSetUnion {}
    #[inline(never)]
    fn pop_class_op(&self, rhs: ast::ClassSet) -> ast::ClassSet {}
}
