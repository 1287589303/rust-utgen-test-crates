pub(crate) type TokenTreeIter = RcVecIntoIter<TokenTree>;
#[cfg(wrap_proc_macro)]
use crate::imp;
#[cfg(span_locations)]
use crate::location::LineColumn;
use crate::parse::{self, Cursor};
use crate::rcvec::{RcVec, RcVecBuilder, RcVecIntoIter, RcVecMut};
use crate::{Delimiter, Spacing, TokenTree};
#[cfg(all(span_locations, not(fuzzing)))]
use alloc::collections::BTreeMap;
#[cfg(all(span_locations, not(fuzzing)))]
use core::cell::RefCell;
#[cfg(span_locations)]
use core::cmp;
use core::fmt::{self, Debug, Display, Write};
use core::mem::ManuallyDrop;
#[cfg(span_locations)]
use core::ops::Range;
use core::ops::RangeBounds;
use core::ptr;
use core::str;
#[cfg(feature = "proc-macro")]
use core::str::FromStr;
use std::ffi::CStr;
#[cfg(wrap_proc_macro)]
use std::panic;
#[cfg(procmacro2_semver_exempt)]
use std::path::PathBuf;
#[derive(Clone)]
pub(crate) struct TokenStream {
    inner: RcVec<TokenTree>,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Cursor<'a> {
    pub(crate) rest: &'a str,
    #[cfg(span_locations)]
    pub(crate) off: u32,
}
pub(crate) struct RcVec<T> {
    inner: Rc<Vec<T>>,
}
#[derive(Debug)]
pub(crate) struct LexError {
    pub(crate) span: Span,
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
impl TokenStream {
    pub(crate) fn new() -> Self {
        TokenStream {
            inner: RcVecBuilder::new().build(),
        }
    }
    pub(crate) fn from_str_checked(src: &str) -> Result<Self, LexError> {
        let mut cursor = get_cursor(src);
        const BYTE_ORDER_MARK: &str = "\u{feff}";
        if cursor.starts_with(BYTE_ORDER_MARK) {
            cursor = cursor.advance(BYTE_ORDER_MARK.len());
        }
        parse::token_stream(cursor)
    }
    #[cfg(feature = "proc-macro")]
    pub(crate) fn from_str_unchecked(src: &str) -> Self {
        Self::from_str_checked(src).unwrap()
    }
    pub(crate) fn is_empty(&self) -> bool {}
    fn take_inner(self) -> RcVecBuilder<TokenTree> {}
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
#[cfg(not(span_locations))]
fn get_cursor(src: &str) -> Cursor {
    Cursor { rest: src }
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
