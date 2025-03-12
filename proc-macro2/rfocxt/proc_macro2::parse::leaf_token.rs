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
pub struct Literal {
    inner: imp::Literal,
    _marker: ProcMacroAutoTraits,
}
#[derive(Clone)]
pub(crate) struct Literal {
    pub(crate) repr: String,
    span: Span,
}
#[derive(Clone)]
pub struct Ident {
    inner: imp::Ident,
    _marker: ProcMacroAutoTraits,
}
#[derive(Clone)]
pub(crate) struct Ident {
    sym: Box<str>,
    span: Span,
    raw: bool,
}
pub(crate) struct Reject;
#[derive(Clone)]
pub struct Punct {
    ch: char,
    spacing: Spacing,
    span: Span,
}
#[derive(Clone)]
pub enum TokenTree {
    /// A token stream surrounded by bracket delimiters.
    Group(Group),
    /// An identifier.
    Ident(Ident),
    /// A single punctuation character (`+`, `,`, `$`, etc.).
    Punct(Punct),
    /// A literal character (`'a'`), string (`"hello"`), number (`2.3`), etc.
    Literal(Literal),
}
#[derive(Clone)]
pub(crate) enum Ident {
    Compiler(proc_macro::Ident),
    Fallback(fallback::Ident),
}
#[derive(Clone)]
pub(crate) enum Literal {
    Compiler(proc_macro::Literal),
    Fallback(fallback::Literal),
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
    fn _new_fallback(inner: fallback::Literal) -> Self {
        Literal {
            inner: imp::Literal::from(inner),
            _marker: MARKER,
        }
    }
    pub fn f64_unsuffixed(f: f64) -> Literal {}
    pub fn f64_suffixed(f: f64) -> Literal {}
    pub fn f32_unsuffixed(f: f32) -> Literal {}
    pub fn f32_suffixed(f: f32) -> Literal {}
    pub fn string(string: &str) -> Literal {}
    pub fn character(ch: char) -> Literal {}
    pub fn byte_character(byte: u8) -> Literal {}
    pub fn byte_string(bytes: &[u8]) -> Literal {}
    pub fn c_string(string: &CStr) -> Literal {}
    pub fn span(&self) -> Span {}
    pub fn set_span(&mut self, span: Span) {}
    pub fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span> {}
    pub unsafe fn from_str_unchecked(repr: &str) -> Self {
        Literal::_new(unsafe { imp::Literal::from_str_unchecked(repr) })
    }
}
fn leaf_token(input: Cursor) -> PResult<TokenTree> {
    if let Ok((input, l)) = literal(input) {
        Ok((input, TokenTree::Literal(crate::Literal::_new_fallback(l))))
    } else if let Ok((input, p)) = punct(input) {
        Ok((input, TokenTree::Punct(p)))
    } else if let Ok((input, i)) = ident(input) {
        Ok((input, TokenTree::Ident(i)))
    } else if input.starts_with(ERROR) {
        let rest = input.advance(ERROR.len());
        let repr = crate::Literal::_new_fallback(Literal::_new(ERROR.to_owned()));
        Ok((rest, TokenTree::Literal(repr)))
    } else {
        Err(Reject)
    }
}
fn punct(input: Cursor) -> PResult<Punct> {
    let (rest, ch) = punct_char(input)?;
    if ch == '\'' {
        if ident_any(rest)?.0.starts_with_char('\'') {
            Err(Reject)
        } else {
            Ok((rest, Punct::new('\'', Spacing::Joint)))
        }
    } else {
        let kind = match punct_char(rest) {
            Ok(_) => Spacing::Joint,
            Err(Reject) => Spacing::Alone,
        };
        Ok((rest, Punct::new(ch, kind)))
    }
}
fn ident(input: Cursor) -> PResult<crate::Ident> {
    if ["r\"", "r#\"", "r##", "b\"", "b\'", "br\"", "br#", "c\"", "cr\"", "cr#"]
        .iter()
        .any(|prefix| input.starts_with(prefix))
    {
        Err(Reject)
    } else {
        ident_any(input)
    }
}
pub(crate) fn literal(input: Cursor) -> PResult<Literal> {
    let rest = literal_nocapture(input)?;
    let end = input.len() - rest.len();
    Ok((rest, Literal::_new(input.rest[..end].to_string())))
}
