use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Hir {
    kind: HirKind,
    is_start_anchored: bool,
    is_match_empty: bool,
    static_explicit_captures_len: Option<usize>,
}
#[derive(Clone, Debug)]
pub(super) struct Parser<'a> {
    /// The configuration of the parser as given by the caller.
    config: Config,
    /// The pattern we're parsing as given by the caller.
    pattern: &'a str,
    /// The call depth of the parser. This is incremented for each
    /// sub-expression parsed. Its peak value is the maximum nesting of the
    /// pattern.
    depth: Cell<u32>,
    /// The current position of the parser.
    pos: Cell<usize>,
    /// The current codepoint of the parser. The codepoint corresponds to the
    /// codepoint encoded in `pattern` beginning at `pos`.
    ///
    /// This is `None` if and only if `pos == pattern.len()`.
    char: Cell<Option<char>>,
    /// The current capture index.
    capture_index: Cell<u32>,
    /// The flags that are currently set.
    flags: RefCell<Flags>,
    /// A sorted sequence of capture names. This is used to detect duplicate
    /// capture names and report an error if one is detected.
    capture_names: RefCell<Vec<String>>,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    /// The maximum number of times we're allowed to recurse.
    ///
    /// Note that unlike the regex-syntax parser, we actually use recursion in
    /// this parser for simplicity. My hope is that by setting a conservative
    /// default call limit and providing a way to configure it, that we can
    /// keep this simplification. But if we must, we can re-work the parser to
    /// put the call stack on the heap like regex-syntax does.
    pub(crate) nest_limit: u32,
    /// Various flags that control how a pattern is interpreted.
    pub(crate) flags: Flags,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    msg: &'static str,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum HirKind {
    Empty,
    Char(char),
    Class(Class),
    Look(Look),
    Repetition(Repetition),
    Capture(Capture),
    Concat(Vec<Hir>),
    Alternation(Vec<Hir>),
}
impl Hir {
    pub(crate) fn parse(config: Config, pattern: &str) -> Result<Hir, Error> {
        self::parse::Parser::new(config, pattern).parse()
    }
    pub(crate) fn kind(&self) -> &HirKind {}
    pub(crate) fn is_start_anchored(&self) -> bool {}
    pub(crate) fn is_match_empty(&self) -> bool {}
    pub(crate) fn static_explicit_captures_len(&self) -> Option<usize> {}
    fn fail() -> Hir {}
    fn empty() -> Hir {}
    fn char(ch: char) -> Hir {}
    fn class(class: Class) -> Hir {}
    fn look(look: Look) -> Hir {}
    fn repetition(rep: Repetition) -> Hir {}
    fn capture(cap: Capture) -> Hir {}
    fn concat(mut subs: Vec<Hir>) -> Hir {}
    fn alternation(mut subs: Vec<Hir>) -> Hir {}
}
impl<'a> Parser<'a> {
    pub(super) fn parse(&self) -> Result<Hir, Error> {
        let hir = self.parse_inner()?;
        check_hir_nesting(&hir, self.config.nest_limit)?;
        Ok(hir)
    }
    fn parse_inner(&self) -> Result<Hir, Error> {}
    fn parse_primitive(&self) -> Result<Hir, Error> {}
    fn parse_escape(&self) -> Result<Hir, Error> {}
    fn maybe_parse_special_word_boundary(&self) -> Result<Option<Hir>, Error> {}
    fn parse_hex(&self) -> Result<Hir, Error> {}
    fn parse_hex_digits(&self, digit_len: usize) -> Result<Hir, Error> {}
    fn parse_hex_brace(&self) -> Result<Hir, Error> {}
    fn parse_decimal(&self) -> Result<u32, Error> {}
    fn parse_uncounted_repetition(
        &self,
        mut concat: Vec<Hir>,
    ) -> Result<Vec<Hir>, Error> {}
    fn parse_counted_repetition(&self, mut concat: Vec<Hir>) -> Result<Vec<Hir>, Error> {}
    fn parse_group(&self) -> Result<Option<Hir>, Error> {}
    fn parse_capture_name(&self) -> Result<&str, Error> {}
    fn parse_flags(&self) -> Result<Flags, Error> {}
    fn parse_flag(&self, flags: &mut Flags, negate: bool) -> Result<(), Error> {}
    fn parse_class(&self) -> Result<Hir, Error> {}
    fn parse_class_range(&self, union: &mut Vec<hir::ClassRange>) -> Result<(), Error> {}
    fn parse_class_item(&self) -> Result<Hir, Error> {}
    fn maybe_parse_posix_class(&self) -> Option<hir::Class> {}
    fn parse_perl_class(&self) -> Hir {}
    fn hir_dot(&self) -> Hir {}
    fn hir_anchor_start(&self) -> Hir {}
    fn hir_anchor_end(&self) -> Hir {}
    fn hir_char(&self, ch: char) -> Hir {}
}
