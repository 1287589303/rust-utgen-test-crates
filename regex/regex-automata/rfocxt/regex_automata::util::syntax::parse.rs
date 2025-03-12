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
impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}
pub fn parse(pattern: &str) -> Result<Hir, Error> {
    parse_with(pattern, &Config::default())
}
pub fn parse_with(pattern: &str, config: &Config) -> Result<Hir, Error> {
    let mut builder = ParserBuilder::new();
    config.apply(&mut builder);
    builder.build().parse(pattern)
}
