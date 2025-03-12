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
