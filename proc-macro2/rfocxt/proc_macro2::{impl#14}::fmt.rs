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
pub struct LexError {
    inner: imp::LexError,
    _marker: ProcMacroAutoTraits,
}
#[derive(Copy, Clone)]
#[cfg_attr(
    all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)),
    derive(PartialEq, Eq)
)]
pub(crate) struct ProcMacroAutoTraits(PhantomData<Rc<()>>);
#[derive(Debug)]
pub(crate) struct LexError {
    pub(crate) span: Span,
}
pub(crate) enum LexError {
    Compiler(proc_macro::LexError),
    Fallback(fallback::LexError),
    CompilerPanic,
}
impl Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
