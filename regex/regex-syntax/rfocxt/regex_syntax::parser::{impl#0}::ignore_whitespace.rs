use crate::{ast, hir, Error};
#[derive(Clone, Debug, Default)]
pub struct ParserBuilder {
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
#[derive(Clone, Debug)]
pub struct ParserBuilder {
    ignore_whitespace: bool,
    nest_limit: u32,
    octal: bool,
    empty_min_range: bool,
}
#[derive(Clone, Debug)]
pub struct TranslatorBuilder {
    utf8: bool,
    line_terminator: u8,
    flags: Flags,
}
impl ParserBuilder {
    pub fn new() -> ParserBuilder {}
    pub fn build(&self) -> Parser {}
    pub fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn utf8(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {
        self.ignore_whitespace = yes;
        self
    }
    pub fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn line_terminator(&mut self, byte: u8) -> &mut ParserBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut ParserBuilder {}
}
