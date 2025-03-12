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
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Cursor<'a> {
    pub(crate) rest: &'a str,
    #[cfg(span_locations)]
    pub(crate) off: u32,
}
#[derive(Clone)]
pub(crate) struct Ident {
    sym: Box<str>,
    span: Span,
    raw: bool,
}
pub(crate) struct Reject;
#[derive(Clone)]
pub(crate) enum Ident {
    Compiler(proc_macro::Ident),
    Fallback(fallback::Ident),
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
