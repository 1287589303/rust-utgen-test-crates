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
pub fn parse_many<P: AsRef<str>>(patterns: &[P]) -> Result<Vec<Hir>, Error> {
    parse_many_with(patterns, &Config::default())
}
pub fn parse_many_with<P: AsRef<str>>(
    patterns: &[P],
    config: &Config,
) -> Result<Vec<Hir>, Error> {
    let mut builder = ParserBuilder::new();
    config.apply(&mut builder);
    let mut hirs = vec![];
    for p in patterns.iter() {
        hirs.push(builder.build().parse(p.as_ref())?);
    }
    Ok(hirs)
}
