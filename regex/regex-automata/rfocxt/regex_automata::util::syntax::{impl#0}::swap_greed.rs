use alloc::{vec, vec::Vec};
use regex_syntax::{
    ast, hir::{self, Hir},
    Error, ParserBuilder,
};
#[derive(Clone, Copy, Debug)]
pub struct Config {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    crlf: bool,
    line_terminator: u8,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    utf8: bool,
    nest_limit: u32,
    octal: bool,
}
impl Config {
    pub fn new() -> Config {}
    pub fn case_insensitive(mut self, yes: bool) -> Config {}
    pub fn multi_line(mut self, yes: bool) -> Config {}
    pub fn dot_matches_new_line(mut self, yes: bool) -> Config {}
    pub fn crlf(mut self, yes: bool) -> Config {}
    pub fn line_terminator(mut self, byte: u8) -> Config {}
    pub fn swap_greed(mut self, yes: bool) -> Config {
        self.swap_greed = yes;
        self
    }
    pub fn ignore_whitespace(mut self, yes: bool) -> Config {}
    pub fn unicode(mut self, yes: bool) -> Config {}
    pub fn utf8(mut self, yes: bool) -> Config {}
    pub fn nest_limit(mut self, limit: u32) -> Config {}
    pub fn octal(mut self, yes: bool) -> Config {}
    pub fn get_unicode(&self) -> bool {}
    pub fn get_case_insensitive(&self) -> bool {}
    pub fn get_multi_line(&self) -> bool {}
    pub fn get_dot_matches_new_line(&self) -> bool {}
    pub fn get_crlf(&self) -> bool {}
    pub fn get_line_terminator(&self) -> u8 {}
    pub fn get_swap_greed(&self) -> bool {}
    pub fn get_ignore_whitespace(&self) -> bool {}
    pub fn get_utf8(&self) -> bool {}
    pub fn get_nest_limit(&self) -> u32 {}
    pub fn get_octal(&self) -> bool {}
    pub(crate) fn apply(&self, builder: &mut ParserBuilder) {}
    pub(crate) fn apply_ast(&self, builder: &mut ast::parse::ParserBuilder) {}
    pub(crate) fn apply_hir(&self, builder: &mut hir::translate::TranslatorBuilder) {}
}
