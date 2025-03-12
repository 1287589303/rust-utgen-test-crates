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
pub(crate) struct Ident {
    sym: Box<str>,
    span: Span,
    raw: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    #[cfg(span_locations)]
    pub(crate) lo: u32,
    #[cfg(span_locations)]
    pub(crate) hi: u32,
}
#[derive(Clone)]
pub(crate) struct Literal {
    pub(crate) repr: String,
    span: Span,
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
impl Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.raw {
            f.write_str("r#")?;
        }
        Display::fmt(&self.sym, f)
    }
}
pub(crate) fn debug_span_field_if_nontrivial(debug: &mut fmt::DebugStruct, span: Span) {
    #[cfg(span_locations)]
    {
        if span.is_call_site() {
            return;
        }
    }
    if cfg!(span_locations) {
        debug.field("span", &span);
    }
}
