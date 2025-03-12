type PResult<'a, O> = Result<(Cursor<'a>, O), Reject>;
use crate::fallback::{
    self, is_ident_continue, is_ident_start, Group, Ident, LexError, Literal, Span,
    TokenStream, TokenStreamBuilder,
};
use crate::{Delimiter, Punct, Spacing, TokenTree};
use core::char;
use core::str::{Bytes, CharIndices, Chars};
const ERROR: &str = "(/*ERROR*/)";
pub(crate) struct TokenStreamBuilder {
    inner: RcVecBuilder<TokenTree>,
}
#[derive(Clone)]
pub struct Punct {
    ch: char,
    spacing: Spacing,
    span: Span,
}
#[derive(Clone)]
pub(crate) struct Literal {
    pub(crate) repr: String,
    span: Span,
}
#[derive(Clone)]
pub struct Group {
    inner: imp::Group,
}
#[derive(Clone)]
pub(crate) struct Group {
    delimiter: Delimiter,
    stream: TokenStream,
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
#[derive(Copy, Clone)]
pub struct Span {
    inner: imp::Span,
    _marker: ProcMacroAutoTraits,
}
#[derive(Clone)]
pub struct Literal {
    inner: imp::Literal,
    _marker: ProcMacroAutoTraits,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    #[cfg(span_locations)]
    pub(crate) lo: u32,
    #[cfg(span_locations)]
    pub(crate) hi: u32,
}
#[derive(Clone)]
pub(crate) struct TokenStream {
    inner: RcVec<TokenTree>,
}
pub(crate) struct Reject;
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Cursor<'a> {
    pub(crate) rest: &'a str,
    #[cfg(span_locations)]
    pub(crate) off: u32,
}
#[derive(Clone)]
pub(crate) enum Literal {
    Compiler(proc_macro::Literal),
    Fallback(fallback::Literal),
}
#[derive(Clone)]
pub(crate) enum Group {
    Compiler(proc_macro::Group),
    Fallback(fallback::Group),
}
#[derive(Clone)]
pub(crate) enum Ident {
    Compiler(proc_macro::Ident),
    Fallback(fallback::Ident),
}
#[derive(Copy, Clone)]
pub(crate) enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Delimiter {
    /// `( ... )`
    Parenthesis,
    /// `{ ... }`
    Brace,
    /// `[ ... ]`
    Bracket,
    /// `∅ ... ∅`
    ///
    /// An invisible delimiter, that may, for example, appear around tokens
    /// coming from a "macro variable" `$var`. It is important to preserve
    /// operator priorities in cases like `$var * 3` where `$var` is `1 + 2`.
    /// Invisible delimiters may not survive roundtrip of a token stream through
    /// a string.
    ///
    /// <div class="warning">
    ///
    /// Note: rustc currently can ignore the grouping of tokens delimited by `None` in the output
    /// of a proc_macro. Only `None`-delimited groups created by a macro_rules macro in the input
    /// of a proc_macro macro are preserved, and only in very specific circumstances.
    /// Any `None`-delimited groups (re)created by a proc_macro will therefore not preserve
    /// operator priorities as indicated above. The other `Delimiter` variants should be used
    /// instead in this context. This is a rustc bug. For details, see
    /// [rust-lang/rust#67062](https://github.com/rust-lang/rust/issues/67062).
    ///
    /// </div>
    None,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Spacing {
    /// E.g. `+` is `Alone` in `+ =`, `+ident` or `+()`.
    Alone,
    /// E.g. `+` is `Joint` in `+=` or `'` is `Joint` in `'#`.
    ///
    /// Additionally, single quote `'` can join with identifiers to form
    /// lifetimes `'ident`.
    Joint,
}
impl TokenStreamBuilder {
    pub(crate) fn new() -> Self {
        TokenStreamBuilder {
            inner: RcVecBuilder::new(),
        }
    }
    pub(crate) fn with_capacity(cap: usize) -> Self {
        TokenStreamBuilder {
            inner: RcVecBuilder::with_capacity(cap),
        }
    }
    pub(crate) fn push_token_from_parser(&mut self, tt: TokenTree) {
        self.inner.push(tt);
    }
    pub(crate) fn build(self) -> TokenStream {
        TokenStream {
            inner: self.inner.build(),
        }
    }
}
impl Punct {
    pub fn new(ch: char, spacing: Spacing) -> Self {
        if let '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | ',' | '-' | '.' | '/'
        | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '^' | '|' | '~' = ch {
            Punct {
                ch,
                spacing,
                span: Span::call_site(),
            }
        } else {
            panic!("unsupported proc macro punctuation character {:?}", ch);
        }
    }
    pub fn as_char(&self) -> char {}
    pub fn spacing(&self) -> Spacing {}
    pub fn span(&self) -> Span {}
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
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
    pub(crate) fn string(string: &str) -> Literal {
        let mut repr = String::with_capacity(string.len() + 2);
        repr.push('"');
        escape_utf8(string, &mut repr);
        repr.push('"');
        Literal::_new(repr)
    }
    pub(crate) fn character(ch: char) -> Literal {}
    pub(crate) fn byte_character(byte: u8) -> Literal {}
    pub(crate) fn byte_string(bytes: &[u8]) -> Literal {}
    pub(crate) fn c_string(string: &CStr) -> Literal {}
    pub(crate) fn span(&self) -> Span {}
    pub fn set_span(&mut self, span: Span) {
        self.inner.set_span(span.inner);
    }
    pub(crate) fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span> {}
}
impl Group {
    fn _new(inner: imp::Group) -> Self {
        Group { inner }
    }
    fn _new_fallback(inner: fallback::Group) -> Self {
        Group {
            inner: imp::Group::from(inner),
        }
    }
    pub(crate) fn new(delimiter: Delimiter, stream: TokenStream) -> Self {
        Group {
            delimiter,
            stream,
            span: Span::call_site(),
        }
    }
    pub fn delimiter(&self) -> Delimiter {}
    pub fn stream(&self) -> TokenStream {}
    pub fn span(&self) -> Span {}
    pub fn span_open(&self) -> Span {}
    pub fn span_close(&self) -> Span {}
    pub fn delim_span(&self) -> DelimSpan {}
    pub(crate) fn set_span(&mut self, span: Span) {
        self.span = span;
    }
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
impl Span {
    fn _new(inner: imp::Span) -> Self {
        Span { inner, _marker: MARKER }
    }
    fn _new_fallback(inner: fallback::Span) -> Self {
        Span {
            inner: imp::Span::from(inner),
            _marker: MARKER,
        }
    }
    pub fn call_site() -> Self {
        Span::_new(imp::Span::call_site())
    }
    pub fn mixed_site() -> Self {
        Span::_new(imp::Span::mixed_site())
    }
    #[cfg(procmacro2_semver_exempt)]
    #[cfg_attr(docsrs, doc(cfg(procmacro2_semver_exempt)))]
    pub fn def_site() -> Self {
        Span::_new(imp::Span::def_site())
    }
    pub fn resolved_at(&self, other: Span) -> Span {}
    pub fn located_at(&self, other: Span) -> Span {}
    #[cfg(wrap_proc_macro)]
    pub fn unwrap(self) -> proc_macro::Span {}
    #[cfg(wrap_proc_macro)]
    pub fn unstable(self) -> proc_macro::Span {}
    #[cfg(all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)))]
    #[cfg_attr(docsrs, doc(cfg(procmacro2_semver_exempt)))]
    pub fn source_file(&self) -> SourceFile {}
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn byte_range(&self) -> Range<usize> {}
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn start(&self) -> LineColumn {}
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn end(&self) -> LineColumn {}
    pub fn join(&self, other: Span) -> Option<Span> {}
    #[cfg(procmacro2_semver_exempt)]
    #[cfg_attr(docsrs, doc(cfg(procmacro2_semver_exempt)))]
    pub fn eq(&self, other: &Span) -> bool {}
    pub fn source_text(&self) -> Option<String> {}
}
fn doc_comment<'a>(
    input: Cursor<'a>,
    trees: &mut TokenStreamBuilder,
) -> PResult<'a, ()> {
    #[cfg(span_locations)]
    let lo = input.off;
    let (rest, (comment, inner)) = doc_comment_contents(input)?;
    let fallback_span = Span {
        #[cfg(span_locations)]
        lo,
        #[cfg(span_locations)]
        hi: rest.off,
    };
    let span = crate::Span::_new_fallback(fallback_span);
    let mut scan_for_bare_cr = comment;
    while let Some(cr) = scan_for_bare_cr.find('\r') {
        let rest = &scan_for_bare_cr[cr + 1..];
        if !rest.starts_with('\n') {
            return Err(Reject);
        }
        scan_for_bare_cr = rest;
    }
    let mut pound = Punct::new('#', Spacing::Alone);
    pound.set_span(span);
    trees.push_token_from_parser(TokenTree::Punct(pound));
    if inner {
        let mut bang = Punct::new('!', Spacing::Alone);
        bang.set_span(span);
        trees.push_token_from_parser(TokenTree::Punct(bang));
    }
    let doc_ident = crate::Ident::_new_fallback(
        Ident::new_unchecked("doc", fallback_span),
    );
    let mut equal = Punct::new('=', Spacing::Alone);
    equal.set_span(span);
    let mut literal = crate::Literal::_new_fallback(Literal::string(comment));
    literal.set_span(span);
    let mut bracketed = TokenStreamBuilder::with_capacity(3);
    bracketed.push_token_from_parser(TokenTree::Ident(doc_ident));
    bracketed.push_token_from_parser(TokenTree::Punct(equal));
    bracketed.push_token_from_parser(TokenTree::Literal(literal));
    let group = Group::new(Delimiter::Bracket, bracketed.build());
    let mut group = crate::Group::_new_fallback(group);
    group.set_span(span);
    trees.push_token_from_parser(TokenTree::Group(group));
    Ok((rest, ()))
}
fn doc_comment_contents(input: Cursor) -> PResult<(&str, bool)> {
    if input.starts_with("//!") {
        let input = input.advance(3);
        let (input, s) = take_until_newline_or_eof(input);
        Ok((input, (s, true)))
    } else if input.starts_with("/*!") {
        let (input, s) = block_comment(input)?;
        Ok((input, (&s[3..s.len() - 2], true)))
    } else if input.starts_with("///") {
        let input = input.advance(3);
        if input.starts_with_char('/') {
            return Err(Reject);
        }
        let (input, s) = take_until_newline_or_eof(input);
        Ok((input, (s, false)))
    } else if input.starts_with("/**") && !input.rest[3..].starts_with('*') {
        let (input, s) = block_comment(input)?;
        Ok((input, (&s[3..s.len() - 2], false)))
    } else {
        Err(Reject)
    }
}
