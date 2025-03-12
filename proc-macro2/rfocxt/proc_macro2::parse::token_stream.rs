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
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Cursor<'a> {
    pub(crate) rest: &'a str,
    #[cfg(span_locations)]
    pub(crate) off: u32,
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
#[derive(Copy, Clone)]
pub struct Span {
    inner: imp::Span,
    _marker: ProcMacroAutoTraits,
}
#[derive(Debug)]
pub(crate) struct LexError {
    pub(crate) span: Span,
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
#[derive(Copy, Clone)]
pub(crate) enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}
#[derive(Clone)]
pub(crate) enum Group {
    Compiler(proc_macro::Group),
    Fallback(fallback::Group),
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
impl TokenTree {
    pub fn span(&self) -> Span {}
    pub fn set_span(&mut self, span: Span) {
        match self {
            TokenTree::Group(t) => t.set_span(span),
            TokenTree::Ident(t) => t.set_span(span),
            TokenTree::Punct(t) => t.set_span(span),
            TokenTree::Literal(t) => t.set_span(span),
        }
    }
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
pub(crate) fn token_stream(mut input: Cursor) -> Result<TokenStream, LexError> {
    let mut trees = TokenStreamBuilder::new();
    let mut stack = Vec::new();
    loop {
        input = skip_whitespace(input);
        if let Ok((rest, ())) = doc_comment(input, &mut trees) {
            input = rest;
            continue;
        }
        #[cfg(span_locations)]
        let lo = input.off;
        let first = match input.bytes().next() {
            Some(first) => first,
            None => {
                match stack.last() {
                    None => return Ok(trees.build()),
                    #[cfg(span_locations)]
                    Some((lo, _frame)) => {
                        return Err(LexError {
                            span: Span { lo: *lo, hi: *lo },
                        });
                    }
                    #[cfg(not(span_locations))]
                    Some(_frame) => return Err(LexError { span: Span {} }),
                }
            }
        };
        if let Some(open_delimiter) = match first {
            b'(' if !input.starts_with(ERROR) => Some(Delimiter::Parenthesis),
            b'[' => Some(Delimiter::Bracket),
            b'{' => Some(Delimiter::Brace),
            _ => None,
        } {
            input = input.advance(1);
            let frame = (open_delimiter, trees);
            #[cfg(span_locations)]
            let frame = (lo, frame);
            stack.push(frame);
            trees = TokenStreamBuilder::new();
        } else if let Some(close_delimiter) = match first {
            b')' => Some(Delimiter::Parenthesis),
            b']' => Some(Delimiter::Bracket),
            b'}' => Some(Delimiter::Brace),
            _ => None,
        } {
            let frame = match stack.pop() {
                Some(frame) => frame,
                None => return Err(lex_error(input)),
            };
            #[cfg(span_locations)]
            let (lo, frame) = frame;
            let (open_delimiter, outer) = frame;
            if open_delimiter != close_delimiter {
                return Err(lex_error(input));
            }
            input = input.advance(1);
            let mut g = Group::new(open_delimiter, trees.build());
            g.set_span(Span {
                #[cfg(span_locations)]
                lo,
                #[cfg(span_locations)]
                hi: input.off,
            });
            trees = outer;
            trees
                .push_token_from_parser(
                    TokenTree::Group(crate::Group::_new_fallback(g)),
                );
        } else {
            let (rest, mut tt) = match leaf_token(input) {
                Ok((rest, tt)) => (rest, tt),
                Err(Reject) => return Err(lex_error(input)),
            };
            tt.set_span(
                crate::Span::_new_fallback(Span {
                    #[cfg(span_locations)]
                    lo,
                    #[cfg(span_locations)]
                    hi: rest.off,
                }),
            );
            trees.push_token_from_parser(tt);
            input = rest;
        }
    }
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
fn skip_whitespace(input: Cursor) -> Cursor {
    let mut s = input;
    while !s.is_empty() {
        let byte = s.as_bytes()[0];
        if byte == b'/' {
            if s.starts_with("//") && (!s.starts_with("///") || s.starts_with("////"))
                && !s.starts_with("//!")
            {
                let (cursor, _) = take_until_newline_or_eof(s);
                s = cursor;
                continue;
            } else if s.starts_with("/**/") {
                s = s.advance(4);
                continue;
            } else if s.starts_with("/*")
                && (!s.starts_with("/**") || s.starts_with("/***"))
                && !s.starts_with("/*!")
            {
                match block_comment(s) {
                    Ok((rest, _)) => {
                        s = rest;
                        continue;
                    }
                    Err(Reject) => return s,
                }
            }
        }
        match byte {
            b' ' | 0x09..=0x0d => {
                s = s.advance(1);
                continue;
            }
            b if b.is_ascii() => {}
            _ => {
                let ch = s.chars().next().unwrap();
                if is_whitespace(ch) {
                    s = s.advance(ch.len_utf8());
                    continue;
                }
            }
        }
        return s;
    }
    s
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
fn lex_error(cursor: Cursor) -> LexError {
    #[cfg(not(span_locations))]
    let _ = cursor;
    LexError {
        span: Span {
            #[cfg(span_locations)]
            lo: cursor.off,
            #[cfg(span_locations)]
            hi: cursor.off,
        },
    }
}
