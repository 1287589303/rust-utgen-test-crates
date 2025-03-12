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
#[derive(Clone)]
pub(crate) struct Literal {
    pub(crate) repr: String,
    span: Span,
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
    fn len(&self) -> usize {
        self.rest.len()
    }
    fn as_bytes(&self) -> &'a [u8] {}
    fn bytes(&self) -> Bytes<'a> {}
    fn chars(&self) -> Chars<'a> {}
    fn char_indices(&self) -> CharIndices<'a> {}
    fn parse(&self, tag: &str) -> Result<Cursor<'a>, Reject> {}
}
impl Literal {
    pub(crate) fn _new(repr: String) -> Self {
        Literal {
            repr,
            span: Span::call_site(),
        }
    }
    pub(crate) fn from_str_checked(repr: &str) -> Result<Self, LexError> {
        let mut cursor = get_cursor(repr);
        #[cfg(span_locations)]
        let lo = cursor.off;
        let negative = cursor.starts_with_char('-');
        if negative {
            cursor = cursor.advance(1);
            if !cursor.starts_with_fn(|ch| ch.is_ascii_digit()) {
                return Err(LexError::call_site());
            }
        }
        if let Ok((rest, mut literal)) = parse::literal(cursor) {
            if rest.is_empty() {
                if negative {
                    literal.repr.insert(0, '-');
                }
                literal.span = Span {
                    #[cfg(span_locations)]
                    lo,
                    #[cfg(span_locations)]
                    hi: rest.off,
                };
                return Ok(literal);
            }
        }
        Err(LexError::call_site())
    }
    pub(crate) unsafe fn from_str_unchecked(repr: &str) -> Self {
        Literal::_new(repr.to_owned())
    }
    pub(crate) fn f32_unsuffixed(f: f32) -> Literal {}
    pub(crate) fn f64_unsuffixed(f: f64) -> Literal {}
    pub(crate) fn string(string: &str) -> Literal {}
    pub(crate) fn character(ch: char) -> Literal {}
    pub(crate) fn byte_character(byte: u8) -> Literal {}
    pub(crate) fn byte_string(bytes: &[u8]) -> Literal {}
    pub(crate) fn c_string(string: &CStr) -> Literal {}
    pub(crate) fn span(&self) -> Span {}
    pub(crate) fn set_span(&mut self, span: Span) {}
    pub(crate) fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span> {}
}
pub(crate) fn literal(input: Cursor) -> PResult<Literal> {
    let rest = literal_nocapture(input)?;
    let end = input.len() - rest.len();
    Ok((rest, Literal::_new(input.rest[..end].to_string())))
}
fn literal_nocapture(input: Cursor) -> Result<Cursor, Reject> {
    if let Ok(ok) = string(input) {
        Ok(ok)
    } else if let Ok(ok) = byte_string(input) {
        Ok(ok)
    } else if let Ok(ok) = c_string(input) {
        Ok(ok)
    } else if let Ok(ok) = byte(input) {
        Ok(ok)
    } else if let Ok(ok) = character(input) {
        Ok(ok)
    } else if let Ok(ok) = float(input) {
        Ok(ok)
    } else if let Ok(ok) = int(input) {
        Ok(ok)
    } else {
        Err(Reject)
    }
}
