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
    pub(crate) fn starts_with(&self, s: &str) -> bool {
        self.rest.starts_with(s)
    }
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
fn digits(mut input: Cursor) -> Result<Cursor, Reject> {
    let base = if input.starts_with("0x") {
        input = input.advance(2);
        16
    } else if input.starts_with("0o") {
        input = input.advance(2);
        8
    } else if input.starts_with("0b") {
        input = input.advance(2);
        2
    } else {
        10
    };
    let mut len = 0;
    let mut empty = true;
    for b in input.bytes() {
        match b {
            b'0'..=b'9' => {
                let digit = (b - b'0') as u64;
                if digit >= base {
                    return Err(Reject);
                }
            }
            b'a'..=b'f' => {
                let digit = 10 + (b - b'a') as u64;
                if digit >= base {
                    break;
                }
            }
            b'A'..=b'F' => {
                let digit = 10 + (b - b'A') as u64;
                if digit >= base {
                    break;
                }
            }
            b'_' => {
                if empty && base == 10 {
                    return Err(Reject);
                }
                len += 1;
                continue;
            }
            _ => break,
        }
        len += 1;
        empty = false;
    }
    if empty { Err(Reject) } else { Ok(input.advance(len)) }
}
