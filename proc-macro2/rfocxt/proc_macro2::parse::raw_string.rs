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
fn delimiter_of_raw_string(input: Cursor) -> PResult<&str> {
    for (i, byte) in input.bytes().enumerate() {
        match byte {
            b'"' => {
                if i > 255 {
                    return Err(Reject);
                }
                return Ok((input.advance(i + 1), &input.rest[..i]));
            }
            b'#' => {}
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
