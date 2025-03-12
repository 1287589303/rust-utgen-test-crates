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
#[derive(Clone)]
pub struct Punct {
    ch: char,
    spacing: Spacing,
    span: Span,
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
#[derive(Clone)]
pub(crate) struct Ident {
    sym: Box<str>,
    span: Span,
    raw: bool,
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
pub enum Spacing {
    /// E.g. `+` is `Alone` in `+ =`, `+ident` or `+()`.
    Alone,
    /// E.g. `+` is `Joint` in `+=` or `'` is `Joint` in `'#`.
    ///
    /// Additionally, single quote `'` can join with identifiers to form
    /// lifetimes `'ident`.
    Joint,
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
#[derive(Clone)]
pub(crate) enum Group {
    Compiler(proc_macro::Group),
    Fallback(fallback::Group),
}
impl Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut joint = false;
        for (i, tt) in self.inner.iter().enumerate() {
            if i != 0 && !joint {
                write!(f, " ")?;
            }
            joint = false;
            match tt {
                TokenTree::Group(tt) => Display::fmt(tt, f),
                TokenTree::Ident(tt) => Display::fmt(tt, f),
                TokenTree::Punct(tt) => {
                    joint = tt.spacing() == Spacing::Joint;
                    Display::fmt(tt, f)
                }
                TokenTree::Literal(tt) => Display::fmt(tt, f),
            }?;
        }
        Ok(())
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
    pub fn spacing(&self) -> Spacing {
        self.spacing
    }
    pub fn span(&self) -> Span {}
    pub fn set_span(&mut self, span: Span) {}
}
impl<T> RcVec<T> {
    pub(crate) fn is_empty(&self) -> bool {}
    pub(crate) fn len(&self) -> usize {}
    pub(crate) fn iter(&self) -> slice::Iter<T> {
        self.inner.iter()
    }
    pub(crate) fn make_mut(&mut self) -> RcVecMut<T>
    where
        T: Clone,
    {}
    pub(crate) fn get_mut(&mut self) -> Option<RcVecMut<T>> {}
    pub(crate) fn make_owned(mut self) -> RcVecBuilder<T>
    where
        T: Clone,
    {}
}
