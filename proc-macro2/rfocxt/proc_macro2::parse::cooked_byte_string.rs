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
    fn bytes(&self) -> Bytes<'a> {
        self.rest.bytes()
    }
    fn chars(&self) -> Chars<'a> {}
    fn char_indices(&self) -> CharIndices<'a> {}
    fn parse(&self, tag: &str) -> Result<Cursor<'a>, Reject> {}
}
fn cooked_byte_string(mut input: Cursor) -> Result<Cursor, Reject> {
    let mut bytes = input.bytes().enumerate();
    while let Some((offset, b)) = bytes.next() {
        match b {
            b'"' => {
                let input = input.advance(offset + 1);
                return Ok(literal_suffix(input));
            }
            b'\r' => {
                match bytes.next() {
                    Some((_, b'\n')) => {}
                    _ => break,
                }
            }
            b'\\' => {
                match bytes.next() {
                    Some((_, b'x')) => {
                        backslash_x_byte(&mut bytes)?;
                    }
                    Some((_, b'n' | b'r' | b't' | b'\\' | b'0' | b'\'' | b'"')) => {}
                    Some((newline, b @ (b'\n' | b'\r'))) => {
                        input = input.advance(newline + 1);
                        trailing_backslash(&mut input, b)?;
                        bytes = input.bytes().enumerate();
                    }
                    _ => break,
                }
            }
            b if b.is_ascii() => {}
            _ => break,
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
