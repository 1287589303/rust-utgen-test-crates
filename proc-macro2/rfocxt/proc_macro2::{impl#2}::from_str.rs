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
#[derive(Debug)]
pub(crate) struct LexError {
    pub(crate) span: Span,
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
pub struct LexError {
    inner: imp::LexError,
    _marker: ProcMacroAutoTraits,
}
pub(crate) enum LexError {
    Compiler(proc_macro::LexError),
    Fallback(fallback::LexError),
    CompilerPanic,
}
#[derive(Clone)]
pub(crate) enum TokenStream {
    Compiler(DeferredTokenStream),
    Fallback(fallback::TokenStream),
}
impl FromStr for TokenStream {
    type Err = LexError;
    fn from_str(src: &str) -> Result<TokenStream, LexError> {
        match imp::TokenStream::from_str_checked(src) {
            Ok(tokens) => Ok(TokenStream::_new(tokens)),
            Err(lex) => {
                Err(LexError {
                    inner: lex,
                    _marker: MARKER,
                })
            }
        }
    }
}
impl TokenStream {
    fn _new(inner: imp::TokenStream) -> Self {
        TokenStream {
            inner,
            _marker: MARKER,
        }
    }
    fn _new_fallback(inner: fallback::TokenStream) -> Self {
        TokenStream {
            inner: imp::TokenStream::from(inner),
            _marker: MARKER,
        }
    }
    pub fn new() -> Self {
        TokenStream::_new(imp::TokenStream::new())
    }
    pub fn is_empty(&self) -> bool {}
}
