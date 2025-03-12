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
pub(crate) struct RcVec<T> {
    inner: Rc<Vec<T>>,
}
pub(crate) struct RcVecMut<'a, T> {
    inner: &'a mut Vec<T>,
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
impl Extend<TokenStream> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenStream>>(&mut self, streams: I) {
        self.inner.make_mut().extend(streams.into_iter().flatten());
    }
}
impl<T> RcVec<T> {
    pub(crate) fn is_empty(&self) -> bool {}
    pub(crate) fn len(&self) -> usize {}
    pub(crate) fn iter(&self) -> slice::Iter<T> {}
    pub(crate) fn make_mut(&mut self) -> RcVecMut<T>
    where
        T: Clone,
    {
        RcVecMut {
            inner: Rc::make_mut(&mut self.inner),
        }
    }
    pub(crate) fn get_mut(&mut self) -> Option<RcVecMut<T>> {}
    pub(crate) fn make_owned(mut self) -> RcVecBuilder<T>
    where
        T: Clone,
    {}
}
