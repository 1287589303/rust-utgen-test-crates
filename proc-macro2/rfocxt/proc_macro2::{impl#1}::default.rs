#[cfg(not(wrap_proc_macro))]
use crate::fallback as imp;
use crate::extra::DelimSpan;
use crate::marker::{ProcMacroAutoTraits, MARKER};
use core::cmp::Ordering;
use core::fmt::{self, Debug, Display};
use core::hash::{Hash, Hasher};
#[cfg(span_locations)]
use core::ops::Range;
use core::ops::RangeBounds;
use core::str::FromStr;
use std::error::Error;
use std::ffi::CStr;
#[cfg(procmacro2_semver_exempt)]
use std::path::PathBuf;
#[cfg(span_locations)]
#[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
pub use crate::location::LineColumn;
#[derive(Clone)]
pub struct TokenStream {
    inner: imp::TokenStream,
    _marker: ProcMacroAutoTraits,
}
#[derive(Clone)]
pub(crate) struct TokenStream {
    inner: RcVec<TokenTree>,
}
#[derive(Copy, Clone)]
#[cfg_attr(
    all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)),
    derive(PartialEq, Eq)
)]
pub(crate) struct ProcMacroAutoTraits(PhantomData<Rc<()>>);
#[derive(Clone)]
pub(crate) enum TokenStream {
    Compiler(DeferredTokenStream),
    Fallback(fallback::TokenStream),
}
impl Default for TokenStream {
    fn default() -> Self {
        TokenStream::new()
    }
}
impl TokenStream {
    pub fn new() -> Self {
        TokenStream::_new(imp::TokenStream::new())
    }
    pub(crate) fn from_str_checked(src: &str) -> Result<Self, LexError> {
        if inside_proc_macro() {
            Ok(
                TokenStream::Compiler(
                    DeferredTokenStream::new(
                        proc_macro::TokenStream::from_str_checked(src)?,
                    ),
                ),
            )
        } else {
            Ok(TokenStream::Fallback(fallback::TokenStream::from_str_checked(src)?))
        }
    }
    pub(crate) fn is_empty(&self) -> bool {}
    fn unwrap_nightly(self) -> proc_macro::TokenStream {}
    fn unwrap_stable(self) -> fallback::TokenStream {}
}
