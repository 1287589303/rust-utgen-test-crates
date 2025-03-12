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
#[derive(Copy, Clone)]
pub struct Span {
    inner: imp::Span,
    _marker: ProcMacroAutoTraits,
}
#[derive(Copy, Clone)]
pub(crate) enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}
impl LexError {
    pub(crate) fn span(&self) -> Span {}
    pub(crate) fn call_site() -> Self {
        LexError {
            span: Span::call_site(),
        }
    }
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
