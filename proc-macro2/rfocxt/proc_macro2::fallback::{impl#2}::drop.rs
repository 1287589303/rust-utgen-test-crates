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
pub(crate) struct RcVecMut<'a, T> {
    inner: &'a mut Vec<T>,
}
pub(crate) struct RcVec<T> {
    inner: Rc<Vec<T>>,
}
#[derive(Clone)]
pub(crate) struct Group {
    delimiter: Delimiter,
    stream: TokenStream,
    span: Span,
}
#[derive(Clone)]
pub struct Group {
    inner: imp::Group,
}
pub(crate) struct RcVecBuilder<T> {
    inner: Vec<T>,
}
#[derive(Clone)]
pub(crate) struct RcVecIntoIter<T> {
    inner: vec::IntoIter<T>,
}
#[derive(Clone)]
pub(crate) enum Group {
    Compiler(proc_macro::Group),
    Fallback(fallback::Group),
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
impl Drop for TokenStream {
    fn drop(&mut self) {
        let mut stack = Vec::new();
        let mut current = match self.inner.get_mut() {
            Some(inner) => inner.take().into_iter(),
            None => return,
        };
        loop {
            while let Some(token) = current.next() {
                let group = match token {
                    TokenTree::Group(group) => group.inner,
                    _ => continue,
                };
                #[cfg(wrap_proc_macro)]
                let group = match group {
                    crate::imp::Group::Fallback(group) => group,
                    crate::imp::Group::Compiler(_) => continue,
                };
                let mut group = group;
                if let Some(inner) = group.stream.inner.get_mut() {
                    stack.push(current);
                    current = inner.take().into_iter();
                }
            }
            match stack.pop() {
                Some(next) => current = next,
                None => return,
            }
        }
    }
}
impl<'a, T> RcVecMut<'a, T> {
    pub(crate) fn push(&mut self, element: T) {}
    pub(crate) fn extend(&mut self, iter: impl IntoIterator<Item = T>) {}
    pub(crate) fn as_mut(&mut self) -> RcVecMut<T> {}
    pub(crate) fn take(self) -> RcVecBuilder<T> {
        let vec = mem::take(self.inner);
        RcVecBuilder { inner: vec }
    }
}
impl<T> RcVec<T> {
    pub(crate) fn is_empty(&self) -> bool {}
    pub(crate) fn len(&self) -> usize {}
    pub(crate) fn iter(&self) -> slice::Iter<T> {}
    pub(crate) fn make_mut(&mut self) -> RcVecMut<T>
    where
        T: Clone,
    {}
    pub(crate) fn get_mut(&mut self) -> Option<RcVecMut<T>> {
        let inner = Rc::get_mut(&mut self.inner)?;
        Some(RcVecMut { inner })
    }
    pub(crate) fn make_owned(mut self) -> RcVecBuilder<T>
    where
        T: Clone,
    {}
}
