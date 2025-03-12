use crate::detection::inside_proc_macro;
use crate::fallback::{self, FromStr2 as _};
#[cfg(span_locations)]
use crate::location::LineColumn;
use crate::{Delimiter, Punct, Spacing, TokenTree};
use core::fmt::{self, Debug, Display};
#[cfg(span_locations)]
use core::ops::Range;
use core::ops::RangeBounds;
use std::ffi::CStr;
#[cfg(super_unstable)]
use std::path::PathBuf;
#[cfg(feature = "proc-macro")]
pub(crate) trait FromStr2: FromStr<Err = proc_macro::LexError> {
    #[cfg(wrap_proc_macro)]
    fn valid(src: &str) -> bool;
    #[cfg(wrap_proc_macro)]
    fn from_str_checked(src: &str) -> Result<Self, imp::LexError> {
        if !Self::valid(src) {
            return Err(imp::LexError::CompilerPanic);
        }
        match panic::catch_unwind(|| Self::from_str(src)) {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(lex)) => Err(imp::LexError::Compiler(lex)),
            Err(_panic) => Err(imp::LexError::CompilerPanic),
        }
    }
    fn from_str_unchecked(src: &str) -> Self {
        Self::from_str(src).unwrap()
    }
}
#[derive(Clone)]
pub(crate) struct TokenStream {
    inner: RcVec<TokenTree>,
}
#[derive(Clone)]
pub struct TokenStream {
    inner: imp::TokenStream,
    _marker: ProcMacroAutoTraits,
}
#[derive(Clone)]
pub(crate) enum TokenStream {
    Compiler(DeferredTokenStream),
    Fallback(fallback::TokenStream),
}
impl From<TokenStream> for proc_macro::TokenStream {
    fn from(inner: TokenStream) -> Self {
        match inner {
            TokenStream::Compiler(inner) => inner.into_token_stream(),
            TokenStream::Fallback(inner) => {
                proc_macro::TokenStream::from_str_unchecked(&inner.to_string())
            }
        }
    }
}
