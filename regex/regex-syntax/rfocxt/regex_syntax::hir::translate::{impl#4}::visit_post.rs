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
    fn start(&mut self) {}
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
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Repetition {
    /// The span of this operation.
    pub span: Span,
    /// The actual operation.
    pub op: RepetitionOp,
    /// Whether this operation was applied greedily or not.
    pub greedy: bool,
    /// The regular expression under repetition.
    pub ast: Box<Ast>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ClassBracketed {
    /// The span of this class.
    pub span: Span,
    /// Whether this class is negated or not. e.g., `[a]` is not negated but
    /// `[^a]` is.
    pub negated: bool,
    /// The type of this set. A set is either a normal union of things, e.g.,
    /// `[abc]` or a result of applying set operations, e.g., `[\pL--c]`.
    pub kind: ClassSet,
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
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
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
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the parser generated the error from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error.
    span: Span,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct SetFlags {
    /// The span of these flags, including the grouping parentheses.
    pub span: Span,
    /// The actual sequence of flags.
    pub flags: Flags,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBytes {
    set: IntervalSet<ClassBytesRange>,
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
pub struct Flags {
    /// The span of this group of flags.
    pub span: Span,
    /// A sequence of flag items. Each item is either a flag or a negation
    /// operator.
    pub items: Vec<FlagsItem>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Assertion {
    /// The span of this assertion.
    pub span: Span,
    /// The assertion kind, e.g., `\b` or `^`.
    pub kind: AssertionKind,
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
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// An error that occurred while translating concrete syntax into abstract
    /// syntax (AST).
    Parse(ast::Error),
    /// An error that occurred while translating abstract syntax into a high
    /// level intermediate representation (HIR).
    Translate(hir::Error),
}
#[derive(Clone, Eq, PartialEq)]
pub enum Class {
    /// A set of characters represented by Unicode scalar values.
    Unicode(ClassUnicode),
    /// A set of characters represented by arbitrary bytes (one byte per
    /// character).
    Bytes(ClassBytes),
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
    #[allow(dead_code)]
    PerlClassNotFound,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Either<Left, Right> {
    Left(Left),
    Right(Right),
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
impl<'t, 'p> Visitor for TranslatorI<'t, 'p> {
    type Output = Hir;
    type Err = Error;
    fn finish(self) -> Result<Hir> {}
    fn visit_pre(&mut self, ast: &Ast) -> Result<()> {}
    fn visit_post(&mut self, ast: &Ast) -> Result<()> {
        match *ast {
            Ast::Empty(_) => {
                self.push(HirFrame::Expr(Hir::empty()));
            }
            Ast::Flags(ref x) => {
                self.set_flags(&x.flags);
                self.push(HirFrame::Expr(Hir::empty()));
            }
            Ast::Literal(ref x) => {
                match self.ast_literal_to_scalar(x)? {
                    Either::Right(byte) => self.push_byte(byte),
                    Either::Left(ch) => {
                        match self.case_fold_char(x.span, ch)? {
                            None => self.push_char(ch),
                            Some(expr) => self.push(HirFrame::Expr(expr)),
                        }
                    }
                }
            }
            Ast::Dot(ref span) => {
                self.push(HirFrame::Expr(self.hir_dot(**span)?));
            }
            Ast::Assertion(ref x) => {
                self.push(HirFrame::Expr(self.hir_assertion(x)?));
            }
            Ast::ClassPerl(ref x) => {
                if self.flags().unicode() {
                    let cls = self.hir_perl_unicode_class(x)?;
                    let hcls = hir::Class::Unicode(cls);
                    self.push(HirFrame::Expr(Hir::class(hcls)));
                } else {
                    let cls = self.hir_perl_byte_class(x)?;
                    let hcls = hir::Class::Bytes(cls);
                    self.push(HirFrame::Expr(Hir::class(hcls)));
                }
            }
            Ast::ClassUnicode(ref x) => {
                let cls = hir::Class::Unicode(self.hir_unicode_class(x)?);
                self.push(HirFrame::Expr(Hir::class(cls)));
            }
            Ast::ClassBracketed(ref ast) => {
                if self.flags().unicode() {
                    let mut cls = self.pop().unwrap().unwrap_class_unicode();
                    self.unicode_fold_and_negate(&ast.span, ast.negated, &mut cls)?;
                    let expr = Hir::class(hir::Class::Unicode(cls));
                    self.push(HirFrame::Expr(expr));
                } else {
                    let mut cls = self.pop().unwrap().unwrap_class_bytes();
                    self.bytes_fold_and_negate(&ast.span, ast.negated, &mut cls)?;
                    let expr = Hir::class(hir::Class::Bytes(cls));
                    self.push(HirFrame::Expr(expr));
                }
            }
            Ast::Repetition(ref x) => {
                let expr = self.pop().unwrap().unwrap_expr();
                self.pop().unwrap().unwrap_repetition();
                self.push(HirFrame::Expr(self.hir_repetition(x, expr)));
            }
            Ast::Group(ref x) => {
                let expr = self.pop().unwrap().unwrap_expr();
                let old_flags = self.pop().unwrap().unwrap_group();
                self.trans().flags.set(old_flags);
                self.push(HirFrame::Expr(self.hir_capture(x, expr)));
            }
            Ast::Concat(_) => {
                let mut exprs = vec![];
                while let Some(expr) = self.pop_concat_expr() {
                    if !matches!(* expr.kind(), HirKind::Empty) {
                        exprs.push(expr);
                    }
                }
                exprs.reverse();
                self.push(HirFrame::Expr(Hir::concat(exprs)));
            }
            Ast::Alternation(_) => {
                let mut exprs = vec![];
                while let Some(expr) = self.pop_alt_expr() {
                    self.pop().unwrap().unwrap_alternation_pipe();
                    exprs.push(expr);
                }
                exprs.reverse();
                self.push(HirFrame::Expr(Hir::alternation(exprs)));
            }
        }
        Ok(())
    }
    fn visit_alternation_in(&mut self) -> Result<()> {}
    fn visit_class_set_item_pre(&mut self, ast: &ast::ClassSetItem) -> Result<()> {}
    fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> Result<()> {}
    fn visit_class_set_binary_op_pre(
        &mut self,
        _op: &ast::ClassSetBinaryOp,
    ) -> Result<()> {}
    fn visit_class_set_binary_op_in(
        &mut self,
        _op: &ast::ClassSetBinaryOp,
    ) -> Result<()> {}
    fn visit_class_set_binary_op_post(
        &mut self,
        op: &ast::ClassSetBinaryOp,
    ) -> Result<()> {}
}
impl<'t, 'p> TranslatorI<'t, 'p> {
    fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {}
    fn trans(&self) -> &Translator {
        &self.trans
    }
    fn push(&self, frame: HirFrame) {
        self.trans().stack.borrow_mut().push(frame);
    }
    fn push_char(&self, ch: char) {
        let mut buf = [0; 4];
        let bytes = ch.encode_utf8(&mut buf).as_bytes();
        let mut stack = self.trans().stack.borrow_mut();
        if let Some(HirFrame::Literal(ref mut literal)) = stack.last_mut() {
            literal.extend_from_slice(bytes);
        } else {
            stack.push(HirFrame::Literal(bytes.to_vec()));
        }
    }
    fn push_byte(&self, byte: u8) {
        let mut stack = self.trans().stack.borrow_mut();
        if let Some(HirFrame::Literal(ref mut literal)) = stack.last_mut() {
            literal.push(byte);
        } else {
            stack.push(HirFrame::Literal(vec![byte]));
        }
    }
    fn pop(&self) -> Option<HirFrame> {
        self.trans().stack.borrow_mut().pop()
    }
    fn pop_concat_expr(&self) -> Option<Hir> {
        let frame = self.pop()?;
        match frame {
            HirFrame::Concat => None,
            HirFrame::Expr(expr) => Some(expr),
            HirFrame::Literal(lit) => Some(Hir::literal(lit)),
            HirFrame::ClassUnicode(_) => {
                unreachable!("expected expr or concat, got Unicode class")
            }
            HirFrame::ClassBytes(_) => {
                unreachable!("expected expr or concat, got byte class")
            }
            HirFrame::Repetition => {
                unreachable!("expected expr or concat, got repetition")
            }
            HirFrame::Group { .. } => unreachable!("expected expr or concat, got group"),
            HirFrame::Alternation => {
                unreachable!("expected expr or concat, got alt marker")
            }
            HirFrame::AlternationBranch => {
                unreachable!("expected expr or concat, got alt branch marker")
            }
        }
    }
    fn pop_alt_expr(&self) -> Option<Hir> {
        let frame = self.pop()?;
        match frame {
            HirFrame::Alternation => None,
            HirFrame::Expr(expr) => Some(expr),
            HirFrame::Literal(lit) => Some(Hir::literal(lit)),
            HirFrame::ClassUnicode(_) => {
                unreachable!("expected expr or alt, got Unicode class")
            }
            HirFrame::ClassBytes(_) => {
                unreachable!("expected expr or alt, got byte class")
            }
            HirFrame::Repetition => unreachable!("expected expr or alt, got repetition"),
            HirFrame::Group { .. } => unreachable!("expected expr or alt, got group"),
            HirFrame::Concat => unreachable!("expected expr or alt, got concat marker"),
            HirFrame::AlternationBranch => {
                unreachable!("expected expr or alt, got alt branch marker")
            }
        }
    }
    fn error(&self, span: Span, kind: ErrorKind) -> Error {}
    fn flags(&self) -> Flags {
        self.trans().flags.get()
    }
    fn set_flags(&self, ast_flags: &ast::Flags) -> Flags {
        let old_flags = self.flags();
        let mut new_flags = Flags::from_ast(ast_flags);
        new_flags.merge(&old_flags);
        self.trans().flags.set(new_flags);
        old_flags
    }
    fn ast_literal_to_scalar(&self, lit: &ast::Literal) -> Result<Either<char, u8>> {
        if self.flags().unicode() {
            return Ok(Either::Left(lit.c));
        }
        let byte = match lit.byte() {
            None => return Ok(Either::Left(lit.c)),
            Some(byte) => byte,
        };
        if byte <= 0x7F {
            return Ok(Either::Left(char::try_from(byte).unwrap()));
        }
        if self.trans().utf8 {
            return Err(self.error(lit.span, ErrorKind::InvalidUtf8));
        }
        Ok(Either::Right(byte))
    }
    fn case_fold_char(&self, span: Span, c: char) -> Result<Option<Hir>> {
        if !self.flags().case_insensitive() {
            return Ok(None);
        }
        if self.flags().unicode() {
            let map = unicode::SimpleCaseFolder::new()
                .map(|f| f.overlaps(c, c))
                .map_err(|_| { self.error(span, ErrorKind::UnicodeCaseUnavailable) })?;
            if !map {
                return Ok(None);
            }
            let mut cls = hir::ClassUnicode::new(
                vec![hir::ClassUnicodeRange::new(c, c,)],
            );
            cls.try_case_fold_simple()
                .map_err(|_| { self.error(span, ErrorKind::UnicodeCaseUnavailable) })?;
            Ok(Some(Hir::class(hir::Class::Unicode(cls))))
        } else {
            if !c.is_ascii() {
                return Ok(None);
            }
            match c {
                'A'..='Z' | 'a'..='z' => {}
                _ => return Ok(None),
            }
            let mut cls = hir::ClassBytes::new(
                vec![
                    hir::ClassBytesRange::new(u8::try_from(c).unwrap(), u8::try_from(c)
                    .unwrap(),)
                ],
            );
            cls.case_fold_simple();
            Ok(Some(Hir::class(hir::Class::Bytes(cls))))
        }
    }
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
    fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {
        let unicode = self.flags().unicode();
        let multi_line = self.flags().multi_line();
        let crlf = self.flags().crlf();
        Ok(
            match asst.kind {
                ast::AssertionKind::StartLine => {
                    Hir::look(
                        if multi_line {
                            if crlf { hir::Look::StartCRLF } else { hir::Look::StartLF }
                        } else {
                            hir::Look::Start
                        },
                    )
                }
                ast::AssertionKind::EndLine => {
                    Hir::look(
                        if multi_line {
                            if crlf { hir::Look::EndCRLF } else { hir::Look::EndLF }
                        } else {
                            hir::Look::End
                        },
                    )
                }
                ast::AssertionKind::StartText => Hir::look(hir::Look::Start),
                ast::AssertionKind::EndText => Hir::look(hir::Look::End),
                ast::AssertionKind::WordBoundary => {
                    Hir::look(
                        if unicode {
                            hir::Look::WordUnicode
                        } else {
                            hir::Look::WordAscii
                        },
                    )
                }
                ast::AssertionKind::NotWordBoundary => {
                    Hir::look(
                        if unicode {
                            hir::Look::WordUnicodeNegate
                        } else {
                            hir::Look::WordAsciiNegate
                        },
                    )
                }
                ast::AssertionKind::WordBoundaryStart
                | ast::AssertionKind::WordBoundaryStartAngle => {
                    Hir::look(
                        if unicode {
                            hir::Look::WordStartUnicode
                        } else {
                            hir::Look::WordStartAscii
                        },
                    )
                }
                ast::AssertionKind::WordBoundaryEnd
                | ast::AssertionKind::WordBoundaryEndAngle => {
                    Hir::look(
                        if unicode {
                            hir::Look::WordEndUnicode
                        } else {
                            hir::Look::WordEndAscii
                        },
                    )
                }
                ast::AssertionKind::WordBoundaryStartHalf => {
                    Hir::look(
                        if unicode {
                            hir::Look::WordStartHalfUnicode
                        } else {
                            hir::Look::WordStartHalfAscii
                        },
                    )
                }
                ast::AssertionKind::WordBoundaryEndHalf => {
                    Hir::look(
                        if unicode {
                            hir::Look::WordEndHalfUnicode
                        } else {
                            hir::Look::WordEndHalfAscii
                        },
                    )
                }
            },
        )
    }
    fn hir_capture(&self, group: &ast::Group, expr: Hir) -> Hir {
        let (index, name) = match group.kind {
            ast::GroupKind::CaptureIndex(index) => (index, None),
            ast::GroupKind::CaptureName { ref name, .. } => {
                (name.index, Some(name.name.clone().into_boxed_str()))
            }
            ast::GroupKind::NonCapturing(_) => return expr,
        };
        Hir::capture(hir::Capture {
            index,
            name,
            sub: Box::new(expr),
        })
    }
    fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
        let (min, max) = match rep.op.kind {
            ast::RepetitionKind::ZeroOrOne => (0, Some(1)),
            ast::RepetitionKind::ZeroOrMore => (0, None),
            ast::RepetitionKind::OneOrMore => (1, None),
            ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(m)) => (m, Some(m)),
            ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(m)) => (m, None),
            ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(m, n)) => {
                (m, Some(n))
            }
        };
        let greedy = if self.flags().swap_greed() { !rep.greedy } else { rep.greedy };
        Hir::repetition(hir::Repetition {
            min,
            max,
            greedy,
            sub: Box::new(expr),
        })
    }
    fn hir_unicode_class(
        &self,
        ast_class: &ast::ClassUnicode,
    ) -> Result<hir::ClassUnicode> {
        use crate::ast::ClassUnicodeKind::*;
        if !self.flags().unicode() {
            return Err(self.error(ast_class.span, ErrorKind::UnicodeNotAllowed));
        }
        let query = match ast_class.kind {
            OneLetter(name) => ClassQuery::OneLetter(name),
            Named(ref name) => ClassQuery::Binary(name),
            NamedValue { ref name, ref value, .. } => {
                ClassQuery::ByValue {
                    property_name: name,
                    property_value: value,
                }
            }
        };
        let mut result = self
            .convert_unicode_class_error(&ast_class.span, unicode::class(query));
        if let Ok(ref mut class) = result {
            self.unicode_fold_and_negate(&ast_class.span, ast_class.negated, class)?;
        }
        result
    }
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
    ) -> Result<hir::ClassBytes> {
        use crate::ast::ClassPerlKind::*;
        assert!(! self.flags().unicode());
        let mut class = match ast_class.kind {
            Digit => hir_ascii_class_bytes(&ast::ClassAsciiKind::Digit),
            Space => hir_ascii_class_bytes(&ast::ClassAsciiKind::Space),
            Word => hir_ascii_class_bytes(&ast::ClassAsciiKind::Word),
        };
        if ast_class.negated {
            class.negate();
        }
        if self.trans().utf8 && !class.is_ascii() {
            return Err(self.error(ast_class.span, ErrorKind::InvalidUtf8));
        }
        Ok(class)
    }
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
    ) -> Result<()> {
        if self.flags().case_insensitive() {
            class
                .try_case_fold_simple()
                .map_err(|_| {
                    self.error(span.clone(), ErrorKind::UnicodeCaseUnavailable)
                })?;
        }
        if negated {
            class.negate();
        }
        Ok(())
    }
    fn bytes_fold_and_negate(
        &self,
        span: &Span,
        negated: bool,
        class: &mut hir::ClassBytes,
    ) -> Result<()> {
        if self.flags().case_insensitive() {
            class.case_fold_simple();
        }
        if negated {
            class.negate();
        }
        if self.trans().utf8 && !class.is_ascii() {
            return Err(self.error(span.clone(), ErrorKind::InvalidUtf8));
        }
        Ok(())
    }
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
impl HirFrame {
    fn unwrap_expr(self) -> Hir {
        match self {
            HirFrame::Expr(expr) => expr,
            HirFrame::Literal(lit) => Hir::literal(lit),
            _ => panic!("tried to unwrap expr from HirFrame, got: {:?}", self),
        }
    }
    fn unwrap_class_unicode(self) -> hir::ClassUnicode {
        match self {
            HirFrame::ClassUnicode(cls) => cls,
            _ => {
                panic!(
                    "tried to unwrap Unicode class \
                 from HirFrame, got: {:?}",
                    self
                )
            }
        }
    }
    fn unwrap_class_bytes(self) -> hir::ClassBytes {
        match self {
            HirFrame::ClassBytes(cls) => cls,
            _ => {
                panic!(
                    "tried to unwrap byte class \
                 from HirFrame, got: {:?}",
                    self
                )
            }
        }
    }
    fn unwrap_repetition(self) {
        match self {
            HirFrame::Repetition => {}
            _ => panic!("tried to unwrap repetition from HirFrame, got: {:?}", self),
        }
    }
    fn unwrap_group(self) -> Flags {
        match self {
            HirFrame::Group { old_flags } => old_flags,
            _ => panic!("tried to unwrap group from HirFrame, got: {:?}", self),
        }
    }
    fn unwrap_alternation_pipe(self) {
        match self {
            HirFrame::AlternationBranch => {}
            _ => panic!("tried to unwrap alt pipe from HirFrame, got: {:?}", self),
        }
    }
}
impl Hir {
    #[inline]
    pub fn empty() -> Hir {
        let props = Properties::empty();
        Hir { kind: HirKind::Empty, props }
    }
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
    pub fn concat(subs: Vec<Hir>) -> Hir {
        let mut new = vec![];
        let mut prior_lit: Option<Vec<u8>> = None;
        for sub in subs {
            let (kind, props) = sub.into_parts();
            match kind {
                HirKind::Literal(Literal(bytes)) => {
                    if let Some(ref mut prior_bytes) = prior_lit {
                        prior_bytes.extend_from_slice(&bytes);
                    } else {
                        prior_lit = Some(bytes.to_vec());
                    }
                }
                HirKind::Concat(subs2) => {
                    for sub2 in subs2 {
                        let (kind2, props2) = sub2.into_parts();
                        match kind2 {
                            HirKind::Literal(Literal(bytes)) => {
                                if let Some(ref mut prior_bytes) = prior_lit {
                                    prior_bytes.extend_from_slice(&bytes);
                                } else {
                                    prior_lit = Some(bytes.to_vec());
                                }
                            }
                            kind2 => {
                                if let Some(prior_bytes) = prior_lit.take() {
                                    new.push(Hir::literal(prior_bytes));
                                }
                                new.push(Hir { kind: kind2, props: props2 });
                            }
                        }
                    }
                }
                HirKind::Empty => {}
                kind => {
                    if let Some(prior_bytes) = prior_lit.take() {
                        new.push(Hir::literal(prior_bytes));
                    }
                    new.push(Hir { kind, props });
                }
            }
        }
        if let Some(prior_bytes) = prior_lit.take() {
            new.push(Hir::literal(prior_bytes));
        }
        if new.is_empty() {
            return Hir::empty();
        } else if new.len() == 1 {
            return new.pop().unwrap();
        }
        let props = Properties::concat(&new);
        Hir {
            kind: HirKind::Concat(new),
            props,
        }
    }
    pub fn alternation(subs: Vec<Hir>) -> Hir {
        let mut new = Vec::with_capacity(subs.len());
        for sub in subs {
            let (kind, props) = sub.into_parts();
            match kind {
                HirKind::Alternation(subs2) => {
                    new.extend(subs2);
                }
                kind => {
                    new.push(Hir { kind, props });
                }
            }
        }
        if new.is_empty() {
            return Hir::fail();
        } else if new.len() == 1 {
            return new.pop().unwrap();
        }
        if let Some(singletons) = singleton_chars(&new) {
            let it = singletons
                .into_iter()
                .map(|ch| ClassUnicodeRange {
                    start: ch,
                    end: ch,
                });
            return Hir::class(Class::Unicode(ClassUnicode::new(it)));
        }
        if let Some(singletons) = singleton_bytes(&new) {
            let it = singletons
                .into_iter()
                .map(|b| ClassBytesRange {
                    start: b,
                    end: b,
                });
            return Hir::class(Class::Bytes(ClassBytes::new(it)));
        }
        if let Some(cls) = class_chars(&new) {
            return Hir::class(cls);
        }
        if let Some(cls) = class_bytes(&new) {
            return Hir::class(cls);
        }
        new = match lift_common_prefix(new) {
            Ok(hir) => return hir,
            Err(unchanged) => unchanged,
        };
        let props = Properties::alternation(&new);
        Hir {
            kind: HirKind::Alternation(new),
            props,
        }
    }
    #[inline]
    pub fn dot(dot: Dot) -> Hir {}
}
