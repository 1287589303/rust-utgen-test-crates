type PResult<'a, O> = Result<(Cursor<'a>, O), Reject>;
use crate::fallback::{
    self, is_ident_continue, is_ident_start, Group, Ident, LexError, Literal, Span,
    TokenStream, TokenStreamBuilder,
};
use crate::{Delimiter, Punct, Spacing, TokenTree};
use core::char;
use core::str::{Bytes, CharIndices, Chars};
const ERROR: &str = "(/*ERROR*/)";
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Cursor<'a> {
    pub(crate) rest: &'a str,
    #[cfg(span_locations)]
    pub(crate) off: u32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    #[cfg(span_locations)]
    pub(crate) lo: u32,
    #[cfg(span_locations)]
    pub(crate) hi: u32,
}
pub(crate) struct Reject;
#[derive(Clone)]
pub(crate) enum Ident {
    Compiler(proc_macro::Ident),
    Fallback(fallback::Ident),
}
impl Ident {
    fn _new(inner: imp::Ident) -> Self {
        Ident { inner, _marker: MARKER }
    }
    fn _new_fallback(inner: fallback::Ident) -> Self {
        Ident {
            inner: imp::Ident::from(inner),
            _marker: MARKER,
        }
    }
    #[track_caller]
    pub fn new(string: &str, span: Span) -> Self {
        Ident::_new(imp::Ident::new_checked(string, span.inner))
    }
    #[track_caller]
    pub fn new_raw(string: &str, span: Span) -> Self {
        Ident::_new(imp::Ident::new_raw_checked(string, span.inner))
    }
    pub fn span(&self) -> Span {}
    pub fn set_span(&mut self, span: Span) {}
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
impl Span {
    #[cfg(span_locations)]
    pub(crate) fn call_site() -> Self {
        Span { lo: 0, hi: 0 }
    }
    #[cfg(span_locations)]
    pub(crate) fn call_site() -> Self {
        Span { lo: 0, hi: 0 }
    }
    pub(crate) fn mixed_site() -> Self {
        Span::call_site()
    }
    #[cfg(procmacro2_semver_exempt)]
    pub(crate) fn def_site() -> Self {
        Span::call_site()
    }
    pub(crate) fn resolved_at(&self, _other: Span) -> Span {}
    pub(crate) fn located_at(&self, other: Span) -> Span {}
    #[cfg(procmacro2_semver_exempt)]
    pub(crate) fn source_file(&self) -> SourceFile {}
    #[cfg(span_locations)]
    pub(crate) fn byte_range(&self) -> Range<usize> {}
    #[cfg(span_locations)]
    pub(crate) fn start(&self) -> LineColumn {}
    #[cfg(span_locations)]
    pub(crate) fn end(&self) -> LineColumn {}
    #[cfg(not(span_locations))]
    pub(crate) fn join(&self, _other: Span) -> Option<Span> {}
    #[cfg(span_locations)]
    pub(crate) fn join(&self, other: Span) -> Option<Span> {}
    #[cfg(not(span_locations))]
    pub(crate) fn source_text(&self) -> Option<String> {}
    #[cfg(span_locations)]
    pub(crate) fn source_text(&self) -> Option<String> {}
    #[cfg(not(span_locations))]
    pub(crate) fn first_byte(self) -> Self {
        self
    }
    #[cfg(span_locations)]
    pub(crate) fn first_byte(self) -> Self {
        Span {
            lo: self.lo,
            hi: cmp::min(self.lo.saturating_add(1), self.hi),
        }
    }
    #[cfg(not(span_locations))]
    pub(crate) fn last_byte(self) -> Self {
        self
    }
    #[cfg(span_locations)]
    pub(crate) fn last_byte(self) -> Self {
        Span {
            lo: cmp::max(self.hi.saturating_sub(1), self.lo),
            hi: self.hi,
        }
    }
    #[cfg(span_locations)]
    fn is_call_site(&self) -> bool {}
}
fn ident_any(input: Cursor) -> PResult<crate::Ident> {
    let raw = input.starts_with("r#");
    let rest = input.advance((raw as usize) << 1);
    let (rest, sym) = ident_not_raw(rest)?;
    if !raw {
        let ident = crate::Ident::_new_fallback(
            Ident::new_unchecked(sym, fallback::Span::call_site()),
        );
        return Ok((rest, ident));
    }
    match sym {
        "_" | "super" | "self" | "Self" | "crate" => return Err(Reject),
        _ => {}
    }
    let ident = crate::Ident::_new_fallback(
        Ident::new_raw_unchecked(sym, fallback::Span::call_site()),
    );
    Ok((rest, ident))
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
