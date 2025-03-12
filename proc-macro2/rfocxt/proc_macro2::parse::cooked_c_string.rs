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
    pub(crate) fn advance(&self, bytes: usize) -> Cursor<'a> {
        let (_front, rest) = self.rest.split_at(bytes);
        Cursor {
            rest,
            #[cfg(span_locations)]
            off: self.off + _front.chars().count() as u32,
        }
    }
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
    fn char_indices(&self) -> CharIndices<'a> {
        self.rest.char_indices()
    }
    fn parse(&self, tag: &str) -> Result<Cursor<'a>, Reject> {}
}
fn cooked_c_string(mut input: Cursor) -> Result<Cursor, Reject> {
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
                        backslash_x_nonzero(&mut chars)?;
                    }
                    Some((_, 'n' | 'r' | 't' | '\\' | '\'' | '"')) => {}
                    Some((_, 'u')) => {
                        if backslash_u(&mut chars)? == '\0' {
                            break;
                        }
                    }
                    Some((newline, ch @ ('\n' | '\r'))) => {
                        input = input.advance(newline + 1);
                        trailing_backslash(&mut input, ch as u8)?;
                        chars = input.char_indices();
                    }
                    _ => break,
                }
            }
            '\0' => break,
            _ch => {}
        }
    }
    Err(Reject)
}
fn literal_suffix(input: Cursor) -> Cursor {
    match ident_not_raw(input) {
        Ok((input, _)) => input,
        Err(Reject) => input,
    }
}
fn trailing_backslash(input: &mut Cursor, mut last: u8) -> Result<(), Reject> {
    let mut whitespace = input.bytes().enumerate();
    loop {
        if last == b'\r' && whitespace.next().map_or(true, |(_, b)| b != b'\n') {
            return Err(Reject);
        }
        match whitespace.next() {
            Some((_, b @ (b' ' | b'\t' | b'\n' | b'\r'))) => {
                last = b;
            }
            Some((offset, _)) => {
                *input = input.advance(offset);
                return Ok(());
            }
            None => return Err(Reject),
        }
    }
}
