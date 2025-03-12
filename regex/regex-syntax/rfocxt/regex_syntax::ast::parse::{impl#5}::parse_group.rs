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
pub struct Flags {
    /// The span of this group of flags.
    pub span: Span,
    /// A sequence of flag items. Each item is either a flag or a negation
    /// operator.
    pub items: Vec<FlagsItem>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct FlagsItem {
    /// The span of this item.
    pub span: Span,
    /// The kind of this item.
    pub kind: FlagsItemKind,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureName {
    /// The span of this capture name.
    pub span: Span,
    /// The capture name.
    pub name: String,
    /// The capture index.
    pub index: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct SetFlags {
    /// The span of these flags, including the grouping parentheses.
    pub span: Span,
    /// The actual sequence of flags.
    pub flags: Flags,
}
#[derive(Clone, Debug)]
pub struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Group {
    /// The span of this group.
    pub span: Span,
    /// The kind of this group.
    pub kind: GroupKind,
    /// The regular expression in this group.
    pub ast: Box<Ast>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Position {
    /// The absolute offset of this position, starting at `0` from the
    /// beginning of the regular expression pattern string.
    pub offset: usize,
    /// The line number, starting at `1`.
    pub line: usize,
    /// The approximate column number, starting at `1`.
    pub column: usize,
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
#[derive(Clone, Debug)]
pub struct Parser {
    /// The current position of the parser.
    pos: Cell<Position>,
    /// The current capture index.
    capture_index: Cell<u32>,
    /// The maximum number of open parens/brackets allowed. If the parser
    /// exceeds this number, then an error is returned.
    nest_limit: u32,
    /// Whether to support octal syntax or not. When `false`, the parser will
    /// return an error helpfully pointing out that backreferences are not
    /// supported.
    octal: bool,
    /// The initial setting for `ignore_whitespace` as provided by
    /// `ParserBuilder`. It is used when resetting the parser's state.
    initial_ignore_whitespace: bool,
    /// Whether the parser supports `{,n}` repetitions as an equivalent to
    /// `{0,n}.`
    empty_min_range: bool,
    /// Whether whitespace should be ignored. When enabled, comments are
    /// also permitted.
    ignore_whitespace: Cell<bool>,
    /// A list of comments, in order of appearance.
    comments: RefCell<Vec<ast::Comment>>,
    /// A stack of grouped sub-expressions, including alternations.
    stack_group: RefCell<Vec<GroupState>>,
    /// A stack of nested character classes. This is only non-empty when
    /// parsing a class.
    stack_class: RefCell<Vec<ClassState>>,
    /// A sorted sequence of capture names. This is used to detect duplicate
    /// capture names and report an error if one is detected.
    capture_names: RefCell<Vec<ast::CaptureName>>,
    /// A scratch buffer used in various places. Mostly this is used to
    /// accumulate relevant characters from parts of a pattern.
    scratch: RefCell<String>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Either<Left, Right> {
    Left(Left),
    Right(Right),
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum GroupKind {
    /// `(a)`
    CaptureIndex(u32),
    /// `(?<name>a)` or `(?P<name>a)`
    CaptureName {
        /// True if the `?P<` syntax is used and false if the `?<` syntax is used.
        starts_with_p: bool,
        /// The capture name.
        name: CaptureName,
    },
    /// `(?:a)` and `(?i:a)`
    NonCapturing(Flags),
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
impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
    fn parse(&self) -> Result<Ast> {}
    fn parse_with_comments(&self) -> Result<ast::WithComments> {}
    #[inline(never)]
    fn parse_uncounted_repetition(
        &self,
        mut concat: ast::Concat,
        kind: ast::RepetitionKind,
    ) -> Result<ast::Concat> {}
    #[inline(never)]
    fn parse_counted_repetition(&self, mut concat: ast::Concat) -> Result<ast::Concat> {}
    #[inline(never)]
    fn parse_group(&self) -> Result<Either<ast::SetFlags, ast::Group>> {
        assert_eq!(self.char(), '(');
        let open_span = self.span_char();
        self.bump();
        self.bump_space();
        if self.is_lookaround_prefix() {
            return Err(
                self
                    .error(
                        Span::new(open_span.start, self.span().end),
                        ast::ErrorKind::UnsupportedLookAround,
                    ),
            );
        }
        let inner_span = self.span();
        let mut starts_with_p = true;
        if self.bump_if("?P<")
            || {
                starts_with_p = false;
                self.bump_if("?<")
            }
        {
            let capture_index = self.next_capture_index(open_span)?;
            let name = self.parse_capture_name(capture_index)?;
            Ok(
                Either::Right(ast::Group {
                    span: open_span,
                    kind: ast::GroupKind::CaptureName {
                        starts_with_p,
                        name,
                    },
                    ast: Box::new(Ast::empty(self.span())),
                }),
            )
        } else if self.bump_if("?") {
            if self.is_eof() {
                return Err(self.error(open_span, ast::ErrorKind::GroupUnclosed));
            }
            let flags = self.parse_flags()?;
            let char_end = self.char();
            self.bump();
            if char_end == ')' {
                if flags.items.is_empty() {
                    return Err(
                        self.error(inner_span, ast::ErrorKind::RepetitionMissing),
                    );
                }
                Ok(
                    Either::Left(ast::SetFlags {
                        span: Span {
                            end: self.pos(),
                            ..open_span
                        },
                        flags,
                    }),
                )
            } else {
                assert_eq!(char_end, ':');
                Ok(
                    Either::Right(ast::Group {
                        span: open_span,
                        kind: ast::GroupKind::NonCapturing(flags),
                        ast: Box::new(Ast::empty(self.span())),
                    }),
                )
            }
        } else {
            let capture_index = self.next_capture_index(open_span)?;
            Ok(
                Either::Right(ast::Group {
                    span: open_span,
                    kind: ast::GroupKind::CaptureIndex(capture_index),
                    ast: Box::new(Ast::empty(self.span())),
                }),
            )
        }
    }
    #[inline(never)]
    fn parse_capture_name(&self, capture_index: u32) -> Result<ast::CaptureName> {
        if self.is_eof() {
            return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
        }
        let start = self.pos();
        loop {
            if self.char() == '>' {
                break;
            }
            if !is_capture_char(self.char(), self.pos() == start) {
                return Err(
                    self.error(self.span_char(), ast::ErrorKind::GroupNameInvalid),
                );
            }
            if !self.bump() {
                break;
            }
        }
        let end = self.pos();
        if self.is_eof() {
            return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
        }
        assert_eq!(self.char(), '>');
        self.bump();
        let name = &self.pattern()[start.offset..end.offset];
        if name.is_empty() {
            return Err(
                self.error(Span::new(start, start), ast::ErrorKind::GroupNameEmpty),
            );
        }
        let capname = ast::CaptureName {
            span: Span::new(start, end),
            name: name.to_string(),
            index: capture_index,
        };
        self.add_capture_name(&capname)?;
        Ok(capname)
    }
    #[inline(never)]
    fn parse_flags(&self) -> Result<ast::Flags> {
        let mut flags = ast::Flags {
            span: self.span(),
            items: vec![],
        };
        let mut last_was_negation = None;
        while self.char() != ':' && self.char() != ')' {
            if self.char() == '-' {
                last_was_negation = Some(self.span_char());
                let item = ast::FlagsItem {
                    span: self.span_char(),
                    kind: ast::FlagsItemKind::Negation,
                };
                if let Some(i) = flags.add_item(item) {
                    return Err(
                        self
                            .error(
                                self.span_char(),
                                ast::ErrorKind::FlagRepeatedNegation {
                                    original: flags.items[i].span,
                                },
                            ),
                    );
                }
            } else {
                last_was_negation = None;
                let item = ast::FlagsItem {
                    span: self.span_char(),
                    kind: ast::FlagsItemKind::Flag(self.parse_flag()?),
                };
                if let Some(i) = flags.add_item(item) {
                    return Err(
                        self
                            .error(
                                self.span_char(),
                                ast::ErrorKind::FlagDuplicate {
                                    original: flags.items[i].span,
                                },
                            ),
                    );
                }
            }
            if !self.bump() {
                return Err(self.error(self.span(), ast::ErrorKind::FlagUnexpectedEof));
            }
        }
        if let Some(span) = last_was_negation {
            return Err(self.error(span, ast::ErrorKind::FlagDanglingNegation));
        }
        flags.span.end = self.pos();
        Ok(flags)
    }
    #[inline(never)]
    fn parse_flag(&self) -> Result<ast::Flag> {}
    fn parse_primitive(&self) -> Result<Primitive> {}
    #[inline(never)]
    fn parse_escape(&self) -> Result<Primitive> {}
    fn maybe_parse_special_word_boundary(
        &self,
        wb_start: Position,
    ) -> Result<Option<ast::AssertionKind>> {}
    #[inline(never)]
    fn parse_octal(&self) -> ast::Literal {}
    #[inline(never)]
    fn parse_hex(&self) -> Result<ast::Literal> {}
    #[inline(never)]
    fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {}
    #[inline(never)]
    fn parse_hex_brace(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {}
    fn parse_decimal(&self) -> Result<u32> {}
    #[inline(never)]
    fn parse_set_class(&self) -> Result<ast::ClassBracketed> {}
    #[inline(never)]
    fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {}
    #[inline(never)]
    fn parse_set_class_item(&self) -> Result<Primitive> {}
    #[inline(never)]
    fn parse_set_class_open(&self) -> Result<(ast::ClassBracketed, ast::ClassSetUnion)> {}
    #[inline(never)]
    fn maybe_parse_ascii_class(&self) -> Option<ast::ClassAscii> {}
    #[inline(never)]
    fn parse_unicode_class(&self) -> Result<ast::ClassUnicode> {}
    #[inline(never)]
    fn parse_perl_class(&self) -> ast::ClassPerl {}
}
impl Span {
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }
    pub fn splat(pos: Position) -> Span {}
    pub fn with_start(self, pos: Position) -> Span {}
    pub fn with_end(self, pos: Position) -> Span {}
    pub fn is_one_line(&self) -> bool {}
    pub fn is_empty(&self) -> bool {}
}
impl Ast {
    pub fn empty(span: Span) -> Ast {
        Ast::Empty(Box::new(span))
    }
    pub fn flags(e: SetFlags) -> Ast {}
    pub fn literal(e: Literal) -> Ast {}
    pub fn dot(span: Span) -> Ast {}
    pub fn assertion(e: Assertion) -> Ast {}
    pub fn class_unicode(e: ClassUnicode) -> Ast {}
    pub fn class_perl(e: ClassPerl) -> Ast {}
    pub fn class_bracketed(e: ClassBracketed) -> Ast {}
    pub fn repetition(e: Repetition) -> Ast {}
    pub fn group(e: Group) -> Ast {}
    pub fn alternation(e: Alternation) -> Ast {}
    pub fn concat(e: Concat) -> Ast {}
    pub fn span(&self) -> &Span {}
    pub fn is_empty(&self) -> bool {}
    fn has_subexprs(&self) -> bool {}
}
