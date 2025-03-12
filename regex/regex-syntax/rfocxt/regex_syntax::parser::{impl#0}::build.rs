use crate::{ast, hir, Error};
#[derive(Clone, Debug, Default)]
pub struct ParserBuilder {
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
#[derive(Clone, Debug)]
pub struct TranslatorBuilder {
    utf8: bool,
    line_terminator: u8,
    flags: Flags,
}
#[derive(Clone, Debug)]
pub struct ParserBuilder {
    ignore_whitespace: bool,
    nest_limit: u32,
    octal: bool,
    empty_min_range: bool,
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
#[derive(Clone, Debug)]
pub struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
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
impl ParserBuilder {
    pub fn new() -> ParserBuilder {}
    pub fn build(&self) -> Parser {
        Parser {
            pos: Cell::new(Position {
                offset: 0,
                line: 1,
                column: 1,
            }),
            capture_index: Cell::new(0),
            nest_limit: self.nest_limit,
            octal: self.octal,
            empty_min_range: self.empty_min_range,
            initial_ignore_whitespace: self.ignore_whitespace,
            ignore_whitespace: Cell::new(self.ignore_whitespace),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        }
    }
    pub fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn utf8(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn line_terminator(&mut self, byte: u8) -> &mut ParserBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut ParserBuilder {}
}
impl TranslatorBuilder {
    pub fn new() -> TranslatorBuilder {}
    pub fn build(&self) -> Translator {
        Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(self.flags),
            utf8: self.utf8,
            line_terminator: self.line_terminator,
        }
    }
    pub fn utf8(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn line_terminator(&mut self, byte: u8) -> &mut TranslatorBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder {}
}
