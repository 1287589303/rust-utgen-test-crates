type PResult<'a, O> = Result<(Cursor<'a>, O), Reject>;
use crate::fallback::{
    self, is_ident_continue, is_ident_start, Group, Ident, LexError, Literal, Span,
    TokenStream, TokenStreamBuilder,
};
use crate::{Delimiter, Punct, Spacing, TokenTree};
use core::char;
use core::str::{Bytes, CharIndices, Chars};
const ERROR: &str = "(/*ERROR*/)";
pub(crate) struct Reject;
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Cursor<'a> {
    pub(crate) rest: &'a str,
    #[cfg(span_locations)]
    pub(crate) off: u32,
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
fn float(input: Cursor) -> Result<Cursor, Reject> {
    let mut rest = float_digits(input)?;
    if let Some(ch) = rest.chars().next() {
        if is_ident_start(ch) {
            rest = ident_not_raw(rest)?.0;
        }
    }
    word_break(rest)
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
fn character(input: Cursor) -> Result<Cursor, Reject> {
    let input = input.parse("'")?;
    let mut chars = input.char_indices();
    let ok = match chars.next().map(|(_, ch)| ch) {
        Some('\\') => {
            match chars.next().map(|(_, ch)| ch) {
                Some('x') => backslash_x_char(&mut chars).is_ok(),
                Some('u') => backslash_u(&mut chars).is_ok(),
                Some('n' | 'r' | 't' | '\\' | '0' | '\'' | '"') => true,
                _ => false,
            }
        }
        ch => ch.is_some(),
    };
    if !ok {
        return Err(Reject);
    }
    let (idx, _) = chars.next().ok_or(Reject)?;
    let input = input.advance(idx).parse("'")?;
    Ok(literal_suffix(input))
}
fn c_string(input: Cursor) -> Result<Cursor, Reject> {
    if let Ok(input) = input.parse("c\"") {
        cooked_c_string(input)
    } else if let Ok(input) = input.parse("cr") {
        raw_c_string(input)
    } else {
        Err(Reject)
    }
}
fn byte(input: Cursor) -> Result<Cursor, Reject> {
    let input = input.parse("b'")?;
    let mut bytes = input.bytes().enumerate();
    let ok = match bytes.next().map(|(_, b)| b) {
        Some(b'\\') => {
            match bytes.next().map(|(_, b)| b) {
                Some(b'x') => backslash_x_byte(&mut bytes).is_ok(),
                Some(b'n' | b'r' | b't' | b'\\' | b'0' | b'\'' | b'"') => true,
                _ => false,
            }
        }
        b => b.is_some(),
    };
    if !ok {
        return Err(Reject);
    }
    let (offset, _) = bytes.next().ok_or(Reject)?;
    if !input.chars().as_str().is_char_boundary(offset) {
        return Err(Reject);
    }
    let input = input.advance(offset).parse("'")?;
    Ok(literal_suffix(input))
}
fn int(input: Cursor) -> Result<Cursor, Reject> {
    let mut rest = digits(input)?;
    if let Some(ch) = rest.chars().next() {
        if is_ident_start(ch) {
            rest = ident_not_raw(rest)?.0;
        }
    }
    word_break(rest)
}
fn byte_string(input: Cursor) -> Result<Cursor, Reject> {
    if let Ok(input) = input.parse("b\"") {
        cooked_byte_string(input)
    } else if let Ok(input) = input.parse("br") {
        raw_byte_string(input)
    } else {
        Err(Reject)
    }
}
