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
pub(crate) struct RcVecMut<'a, T> {
    inner: &'a mut Vec<T>,
}
#[derive(Clone)]
pub(crate) struct Literal {
    pub(crate) repr: String,
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
impl<'a, T> RcVecMut<'a, T> {
    pub(crate) fn push(&mut self, element: T) {
        self.inner.push(element);
    }
    pub(crate) fn extend(&mut self, iter: impl IntoIterator<Item = T>) {}
    pub(crate) fn as_mut(&mut self) -> RcVecMut<T> {}
    pub(crate) fn take(self) -> RcVecBuilder<T> {}
}
fn push_token_from_proc_macro(mut vec: RcVecMut<TokenTree>, token: TokenTree) {
    match token {
        TokenTree::Literal(
            crate::Literal {
                #[cfg(wrap_proc_macro)]
                inner: crate::imp::Literal::Fallback(literal),
                #[cfg(not(wrap_proc_macro))]
                inner: literal,
                ..
            },
        ) if literal.repr.starts_with('-') => {
            push_negative_literal(vec, literal);
        }
        _ => vec.push(token),
    }
    #[cold]
    fn push_negative_literal(mut vec: RcVecMut<TokenTree>, mut literal: Literal) {
        literal.repr.remove(0);
        let mut punct = crate::Punct::new('-', Spacing::Alone);
        punct.set_span(crate::Span::_new_fallback(literal.span));
        vec.push(TokenTree::Punct(punct));
        vec.push(TokenTree::Literal(crate::Literal::_new_fallback(literal)));
    }
}
#[cold]
fn push_negative_literal(mut vec: RcVecMut<TokenTree>, mut literal: Literal) {
    literal.repr.remove(0);
    let mut punct = crate::Punct::new('-', Spacing::Alone);
    punct.set_span(crate::Span::_new_fallback(literal.span));
    vec.push(TokenTree::Punct(punct));
    vec.push(TokenTree::Literal(crate::Literal::_new_fallback(literal)));
}
