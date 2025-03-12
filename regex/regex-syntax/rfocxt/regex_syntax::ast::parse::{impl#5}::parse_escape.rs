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
#[derive(Clone, Debug)]
pub struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
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
pub struct Assertion {
    /// The span of this assertion.
    pub span: Span,
    /// The assertion kind, e.g., `\b` or `^`.
    pub kind: AssertionKind,
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
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the parser generated the error from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error.
    span: Span,
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
pub enum SpecialLiteralKind {
    /// Bell, spelled `\a` (`\x07`).
    Bell,
    /// Form feed, spelled `\f` (`\x0C`).
    FormFeed,
    /// Tab, spelled `\t` (`\x09`).
    Tab,
    /// Line feed, spelled `\n` (`\x0A`).
    LineFeed,
    /// Carriage return, spelled `\r` (`\x0D`).
    CarriageReturn,
    /// Vertical tab, spelled `\v` (`\x0B`).
    VerticalTab,
    /// Space, spelled `\ ` (`\x20`). Note that this can only appear when
    /// parsing in verbose mode.
    Space,
}
#[derive(Clone, Debug, Eq, PartialEq)]
enum Primitive {
    Literal(ast::Literal),
    Assertion(ast::Assertion),
    Dot(Span),
    Perl(ast::ClassPerl),
    Unicode(ast::ClassUnicode),
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum AssertionKind {
    /// `^`
    StartLine,
    /// `$`
    EndLine,
    /// `\A`
    StartText,
    /// `\z`
    EndText,
    /// `\b`
    WordBoundary,
    /// `\B`
    NotWordBoundary,
    /// `\b{start}`
    WordBoundaryStart,
    /// `\b{end}`
    WordBoundaryEnd,
    /// `\<` (alias for `\b{start}`)
    WordBoundaryStartAngle,
    /// `\>` (alias for `\b{end}`)
    WordBoundaryEndAngle,
    /// `\b{start-half}`
    WordBoundaryStartHalf,
    /// `\b{end-half}`
    WordBoundaryEndHalf,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum LiteralKind {
    /// The literal is written verbatim, e.g., `a` or `☃`.
    Verbatim,
    /// The literal is written as an escape because it is otherwise a special
    /// regex meta character, e.g., `\*` or `\[`.
    Meta,
    /// The literal is written as an escape despite the fact that the escape is
    /// unnecessary, e.g., `\%` or `\/`.
    Superfluous,
    /// The literal is written as an octal escape, e.g., `\141`.
    Octal,
    /// The literal is written as a hex code with a fixed number of digits
    /// depending on the type of the escape, e.g., `\x61` or `\u0061` or
    /// `\U00000061`.
    HexFixed(HexLiteralKind),
    /// The literal is written as a hex code with a bracketed number of
    /// digits. The only restriction is that the bracketed hex code must refer
    /// to a valid Unicode scalar value.
    HexBrace(HexLiteralKind),
    /// The literal is written as a specially recognized escape, e.g., `\f`
    /// or `\n`.
    Special(SpecialLiteralKind),
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
    fn parse_group(&self) -> Result<Either<ast::SetFlags, ast::Group>> {}
    #[inline(never)]
    fn parse_capture_name(&self, capture_index: u32) -> Result<ast::CaptureName> {}
    #[inline(never)]
    fn parse_flags(&self) -> Result<ast::Flags> {}
    #[inline(never)]
    fn parse_flag(&self) -> Result<ast::Flag> {}
    fn parse_primitive(&self) -> Result<Primitive> {}
    #[inline(never)]
    fn parse_escape(&self) -> Result<Primitive> {
        assert_eq!(self.char(), '\\');
        let start = self.pos();
        if !self.bump() {
            return Err(
                self
                    .error(
                        Span::new(start, self.pos()),
                        ast::ErrorKind::EscapeUnexpectedEof,
                    ),
            );
        }
        let c = self.char();
        match c {
            '0'..='7' => {
                if !self.parser().octal {
                    return Err(
                        self
                            .error(
                                Span::new(start, self.span_char().end),
                                ast::ErrorKind::UnsupportedBackreference,
                            ),
                    );
                }
                let mut lit = self.parse_octal();
                lit.span.start = start;
                return Ok(Primitive::Literal(lit));
            }
            '8'..='9' if !self.parser().octal => {
                return Err(
                    self
                        .error(
                            Span::new(start, self.span_char().end),
                            ast::ErrorKind::UnsupportedBackreference,
                        ),
                );
            }
            'x' | 'u' | 'U' => {
                let mut lit = self.parse_hex()?;
                lit.span.start = start;
                return Ok(Primitive::Literal(lit));
            }
            'p' | 'P' => {
                let mut cls = self.parse_unicode_class()?;
                cls.span.start = start;
                return Ok(Primitive::Unicode(cls));
            }
            'd' | 's' | 'w' | 'D' | 'S' | 'W' => {
                let mut cls = self.parse_perl_class();
                cls.span.start = start;
                return Ok(Primitive::Perl(cls));
            }
            _ => {}
        }
        self.bump();
        let span = Span::new(start, self.pos());
        if is_meta_character(c) {
            return Ok(
                Primitive::Literal(ast::Literal {
                    span,
                    kind: ast::LiteralKind::Meta,
                    c,
                }),
            );
        }
        if is_escapeable_character(c) {
            return Ok(
                Primitive::Literal(ast::Literal {
                    span,
                    kind: ast::LiteralKind::Superfluous,
                    c,
                }),
            );
        }
        let special = |kind, c| {
            Ok(
                Primitive::Literal(ast::Literal {
                    span,
                    kind: ast::LiteralKind::Special(kind),
                    c,
                }),
            )
        };
        match c {
            'a' => special(ast::SpecialLiteralKind::Bell, '\x07'),
            'f' => special(ast::SpecialLiteralKind::FormFeed, '\x0C'),
            't' => special(ast::SpecialLiteralKind::Tab, '\t'),
            'n' => special(ast::SpecialLiteralKind::LineFeed, '\n'),
            'r' => special(ast::SpecialLiteralKind::CarriageReturn, '\r'),
            'v' => special(ast::SpecialLiteralKind::VerticalTab, '\x0B'),
            'A' => {
                Ok(
                    Primitive::Assertion(ast::Assertion {
                        span,
                        kind: ast::AssertionKind::StartText,
                    }),
                )
            }
            'z' => {
                Ok(
                    Primitive::Assertion(ast::Assertion {
                        span,
                        kind: ast::AssertionKind::EndText,
                    }),
                )
            }
            'b' => {
                let mut wb = ast::Assertion {
                    span,
                    kind: ast::AssertionKind::WordBoundary,
                };
                if !self.is_eof() && self.char() == '{' {
                    if let Some(kind) = self.maybe_parse_special_word_boundary(start)? {
                        wb.kind = kind;
                        wb.span.end = self.pos();
                    }
                }
                Ok(Primitive::Assertion(wb))
            }
            'B' => {
                Ok(
                    Primitive::Assertion(ast::Assertion {
                        span,
                        kind: ast::AssertionKind::NotWordBoundary,
                    }),
                )
            }
            '<' => {
                Ok(
                    Primitive::Assertion(ast::Assertion {
                        span,
                        kind: ast::AssertionKind::WordBoundaryStartAngle,
                    }),
                )
            }
            '>' => {
                Ok(
                    Primitive::Assertion(ast::Assertion {
                        span,
                        kind: ast::AssertionKind::WordBoundaryEndAngle,
                    }),
                )
            }
            _ => Err(self.error(span, ast::ErrorKind::EscapeUnrecognized)),
        }
    }
    fn maybe_parse_special_word_boundary(
        &self,
        wb_start: Position,
    ) -> Result<Option<ast::AssertionKind>> {
        assert_eq!(self.char(), '{');
        let is_valid_char = |c| match c {
            'A'..='Z' | 'a'..='z' | '-' => true,
            _ => false,
        };
        let start = self.pos();
        if !self.bump_and_bump_space() {
            return Err(
                self
                    .error(
                        Span::new(wb_start, self.pos()),
                        ast::ErrorKind::SpecialWordOrRepetitionUnexpectedEof,
                    ),
            );
        }
        let start_contents = self.pos();
        if !is_valid_char(self.char()) {
            self.parser().pos.set(start);
            return Ok(None);
        }
        let mut scratch = self.parser().scratch.borrow_mut();
        scratch.clear();
        while !self.is_eof() && is_valid_char(self.char()) {
            scratch.push(self.char());
            self.bump_and_bump_space();
        }
        if self.is_eof() || self.char() != '}' {
            return Err(
                self
                    .error(
                        Span::new(start, self.pos()),
                        ast::ErrorKind::SpecialWordBoundaryUnclosed,
                    ),
            );
        }
        let end = self.pos();
        self.bump();
        let kind = match scratch.as_str() {
            "start" => ast::AssertionKind::WordBoundaryStart,
            "end" => ast::AssertionKind::WordBoundaryEnd,
            "start-half" => ast::AssertionKind::WordBoundaryStartHalf,
            "end-half" => ast::AssertionKind::WordBoundaryEndHalf,
            _ => {
                return Err(
                    self
                        .error(
                            Span::new(start_contents, end),
                            ast::ErrorKind::SpecialWordBoundaryUnrecognized,
                        ),
                );
            }
        };
        Ok(Some(kind))
    }
    #[inline(never)]
    fn parse_octal(&self) -> ast::Literal {
        assert!(self.parser().octal);
        assert!('0' <= self.char() && self.char() <= '7');
        let start = self.pos();
        while self.bump() && '0' <= self.char() && self.char() <= '7'
            && self.pos().offset - start.offset <= 2
        {}
        let end = self.pos();
        let octal = &self.pattern()[start.offset..end.offset];
        let codepoint = u32::from_str_radix(octal, 8).expect("valid octal number");
        let c = char::from_u32(codepoint).expect("Unicode scalar value");
        ast::Literal {
            span: Span::new(start, end),
            kind: ast::LiteralKind::Octal,
            c,
        }
    }
    #[inline(never)]
    fn parse_hex(&self) -> Result<ast::Literal> {
        assert!(self.char() == 'x' || self.char() == 'u' || self.char() == 'U');
        let hex_kind = match self.char() {
            'x' => ast::HexLiteralKind::X,
            'u' => ast::HexLiteralKind::UnicodeShort,
            _ => ast::HexLiteralKind::UnicodeLong,
        };
        if !self.bump_and_bump_space() {
            return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
        }
        if self.char() == '{' {
            self.parse_hex_brace(hex_kind)
        } else {
            self.parse_hex_digits(hex_kind)
        }
    }
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
    fn parse_unicode_class(&self) -> Result<ast::ClassUnicode> {
        assert!(self.char() == 'p' || self.char() == 'P');
        let mut scratch = self.parser().scratch.borrow_mut();
        scratch.clear();
        let negated = self.char() == 'P';
        if !self.bump_and_bump_space() {
            return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
        }
        let (start, kind) = if self.char() == '{' {
            let start = self.span_char().end;
            while self.bump_and_bump_space() && self.char() != '}' {
                scratch.push(self.char());
            }
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }
            assert_eq!(self.char(), '}');
            self.bump();
            let name = scratch.as_str();
            if let Some(i) = name.find("!=") {
                (
                    start,
                    ast::ClassUnicodeKind::NamedValue {
                        op: ast::ClassUnicodeOpKind::NotEqual,
                        name: name[..i].to_string(),
                        value: name[i + 2..].to_string(),
                    },
                )
            } else if let Some(i) = name.find(':') {
                (
                    start,
                    ast::ClassUnicodeKind::NamedValue {
                        op: ast::ClassUnicodeOpKind::Colon,
                        name: name[..i].to_string(),
                        value: name[i + 1..].to_string(),
                    },
                )
            } else if let Some(i) = name.find('=') {
                (
                    start,
                    ast::ClassUnicodeKind::NamedValue {
                        op: ast::ClassUnicodeOpKind::Equal,
                        name: name[..i].to_string(),
                        value: name[i + 1..].to_string(),
                    },
                )
            } else {
                (start, ast::ClassUnicodeKind::Named(name.to_string()))
            }
        } else {
            let start = self.pos();
            let c = self.char();
            if c == '\\' {
                return Err(
                    self.error(self.span_char(), ast::ErrorKind::UnicodeClassInvalid),
                );
            }
            self.bump_and_bump_space();
            let kind = ast::ClassUnicodeKind::OneLetter(c);
            (start, kind)
        };
        Ok(ast::ClassUnicode {
            span: Span::new(start, self.pos()),
            negated,
            kind,
        })
    }
    #[inline(never)]
    fn parse_perl_class(&self) -> ast::ClassPerl {
        let c = self.char();
        let span = self.span_char();
        self.bump();
        let (negated, kind) = match c {
            'd' => (false, ast::ClassPerlKind::Digit),
            'D' => (true, ast::ClassPerlKind::Digit),
            's' => (false, ast::ClassPerlKind::Space),
            'S' => (true, ast::ClassPerlKind::Space),
            'w' => (false, ast::ClassPerlKind::Word),
            'W' => (true, ast::ClassPerlKind::Word),
            c => panic!("expected valid Perl class but got '{}'", c),
        };
        ast::ClassPerl {
            span,
            kind,
            negated,
        }
    }
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
pub fn is_escapeable_character(c: char) -> bool {
    if is_meta_character(c) {
        return true;
    }
    if !c.is_ascii() {
        return false;
    }
    match c {
        '0'..='9' | 'A'..='Z' | 'a'..='z' => false,
        '<' | '>' => false,
        _ => true,
    }
}
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}
