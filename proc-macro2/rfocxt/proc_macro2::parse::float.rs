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
    fn chars(&self) -> Chars<'a> {
        self.rest.chars()
    }
    fn char_indices(&self) -> CharIndices<'a> {}
    fn parse(&self, tag: &str) -> Result<Cursor<'a>, Reject> {}
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
pub(crate) fn is_ident_start(c: char) -> bool {
    c == '_' || unicode_ident::is_xid_start(c)
}
fn ident_not_raw(input: Cursor) -> PResult<&str> {
    let mut chars = input.char_indices();
    match chars.next() {
        Some((_, ch)) if is_ident_start(ch) => {}
        _ => return Err(Reject),
    }
    let mut end = input.len();
    for (i, ch) in chars {
        if !is_ident_continue(ch) {
            end = i;
            break;
        }
    }
    Ok((input.advance(end), &input.rest[..end]))
}
fn word_break(input: Cursor) -> Result<Cursor, Reject> {
    match input.chars().next() {
        Some(ch) if is_ident_continue(ch) => Err(Reject),
        Some(_) | None => Ok(input),
    }
}
fn float_digits(input: Cursor) -> Result<Cursor, Reject> {
    let mut chars = input.chars().peekable();
    match chars.next() {
        Some(ch) if '0' <= ch && ch <= '9' => {}
        _ => return Err(Reject),
    }
    let mut len = 1;
    let mut has_dot = false;
    let mut has_exp = false;
    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' | '_' => {
                chars.next();
                len += 1;
            }
            '.' => {
                if has_dot {
                    break;
                }
                chars.next();
                if chars.peek().map_or(false, |&ch| ch == '.' || is_ident_start(ch)) {
                    return Err(Reject);
                }
                len += 1;
                has_dot = true;
            }
            'e' | 'E' => {
                chars.next();
                len += 1;
                has_exp = true;
                break;
            }
            _ => break,
        }
    }
    if !(has_dot || has_exp) {
        return Err(Reject);
    }
    if has_exp {
        let token_before_exp = if has_dot {
            Ok(input.advance(len - 1))
        } else {
            Err(Reject)
        };
        let mut has_sign = false;
        let mut has_exp_value = false;
        while let Some(&ch) = chars.peek() {
            match ch {
                '+' | '-' => {
                    if has_exp_value {
                        break;
                    }
                    if has_sign {
                        return token_before_exp;
                    }
                    chars.next();
                    len += 1;
                    has_sign = true;
                }
                '0'..='9' => {
                    chars.next();
                    len += 1;
                    has_exp_value = true;
                }
                '_' => {
                    chars.next();
                    len += 1;
                }
                _ => break,
            }
        }
        if !has_exp_value {
            return token_before_exp;
        }
    }
    Ok(input.advance(len))
}
