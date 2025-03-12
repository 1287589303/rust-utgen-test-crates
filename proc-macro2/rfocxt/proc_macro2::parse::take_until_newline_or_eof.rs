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
    fn len(&self) -> usize {
        self.rest.len()
    }
    fn as_bytes(&self) -> &'a [u8] {}
    fn bytes(&self) -> Bytes<'a> {}
    fn chars(&self) -> Chars<'a> {}
    fn char_indices(&self) -> CharIndices<'a> {
        self.rest.char_indices()
    }
    fn parse(&self, tag: &str) -> Result<Cursor<'a>, Reject> {}
}
fn take_until_newline_or_eof(input: Cursor) -> (Cursor, &str) {
    let chars = input.char_indices();
    for (i, ch) in chars {
        if ch == '\n' {
            return (input.advance(i), &input.rest[..i]);
        } else if ch == '\r' && input.rest[i + 1..].starts_with('\n') {
            return (input.advance(i + 1), &input.rest[..i]);
        }
    }
    (input.advance(input.len()), input.rest)
}
