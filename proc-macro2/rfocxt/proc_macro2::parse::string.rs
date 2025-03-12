type PResult<'a, O> = Result<(Cursor<'a>, O), Reject>;
use crate::fallback::{
    self, is_ident_continue, is_ident_start, Group, Ident, LexError, Literal, Span,
    TokenStream, TokenStreamBuilder,
};
use crate::{Delimiter, Punct, Spacing, TokenTree};
use core::char;
use core::str::{Bytes, CharIndices, Chars};
const ERROR: &str = "(/*ERROR*/)";
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Cursor<'a> {
    pub(crate) rest: &'a str,
    #[cfg(span_locations)]
    pub(crate) off: u32,
}
pub(crate) struct Reject;
impl<'a> Cursor<'a> {
    pub(crate) fn advance(&self, bytes: usize) -> Cursor<'a> {}
    pub(crate) fn starts_with(&self, s: &str) -> bool {}
    pub(crate) fn starts_with_char(&self, ch: char) -> bool {}
    pub(crate) fn starts_with_fn<Pattern>(&self, f: Pattern) -> bool
    where
        Pattern: FnMut(char) -> bool,
    {}
    pub(crate) fn is_empty(&self) -> bool {}
    fn len(&self) -> usize {}
    fn as_bytes(&self) -> &'a [u8] {}
    fn bytes(&self) -> Bytes<'a> {}
    fn chars(&self) -> Chars<'a> {}
    fn char_indices(&self) -> CharIndices<'a> {}
    fn parse(&self, tag: &str) -> Result<Cursor<'a>, Reject> {
        if self.starts_with(tag) { Ok(self.advance(tag.len())) } else { Err(Reject) }
    }
}
fn string(input: Cursor) -> Result<Cursor, Reject> {
    if let Ok(input) = input.parse("\"") {
        cooked_string(input)
    } else if let Ok(input) = input.parse("r") {
        raw_string(input)
    } else {
        Err(Reject)
    }
}
fn cooked_string(mut input: Cursor) -> Result<Cursor, Reject> {
    let mut chars = input.char_indices();
    while let Some((i, ch)) = chars.next() {
        match ch {
            '"' => {
                let input = input.advance(i + 1);
                return Ok(literal_suffix(input));
            }
            '\r' => {
                match chars.next() {
                    Some((_, '\n')) => {}
                    _ => break,
                }
            }
            '\\' => {
                match chars.next() {
                    Some((_, 'x')) => {
                        backslash_x_char(&mut chars)?;
                    }
                    Some((_, 'n' | 'r' | 't' | '\\' | '\'' | '"' | '0')) => {}
                    Some((_, 'u')) => {
                        backslash_u(&mut chars)?;
                    }
                    Some((newline, ch @ ('\n' | '\r'))) => {
                        input = input.advance(newline + 1);
                        trailing_backslash(&mut input, ch as u8)?;
                        chars = input.char_indices();
                    }
                    _ => break,
                }
            }
            _ch => {}
        }
    }
    Err(Reject)
}
fn raw_string(input: Cursor) -> Result<Cursor, Reject> {
    let (input, delimiter) = delimiter_of_raw_string(input)?;
    let mut bytes = input.bytes().enumerate();
    while let Some((i, byte)) = bytes.next() {
        match byte {
            b'"' if input.rest[i + 1..].starts_with(delimiter) => {
                let rest = input.advance(i + 1 + delimiter.len());
                return Ok(literal_suffix(rest));
            }
            b'\r' => {
                match bytes.next() {
                    Some((_, b'\n')) => {}
                    _ => break,
                }
            }
            _ => {}
        }
    }
    Err(Reject)
}
