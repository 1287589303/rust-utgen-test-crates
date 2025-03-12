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
#[cfg(all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)))]
#[cfg_attr(docsrs, doc(cfg(procmacro2_semver_exempt)))]
#[derive(Clone, PartialEq, Eq)]
pub struct SourceFile {
    inner: imp::SourceFile,
    _marker: ProcMacroAutoTraits,
}
#[cfg(procmacro2_semver_exempt)]
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct SourceFile {
    path: PathBuf,
}
#[derive(Copy, Clone)]
pub struct Span {
    inner: imp::Span,
    _marker: ProcMacroAutoTraits,
}
#[derive(Copy, Clone)]
#[cfg_attr(
    all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)),
    derive(PartialEq, Eq)
)]
pub(crate) struct ProcMacroAutoTraits(PhantomData<Rc<()>>);
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    #[cfg(span_locations)]
    pub(crate) lo: u32,
    #[cfg(span_locations)]
    pub(crate) hi: u32,
}
#[derive(Copy, Clone)]
pub(crate) enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}
#[derive(Clone, PartialEq, Eq)]
#[cfg(super_unstable)]
pub(crate) enum SourceFile {
    Compiler(proc_macro::SourceFile),
    Fallback(fallback::SourceFile),
}
#[cfg(all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)))]
impl SourceFile {
    fn _new(inner: imp::SourceFile) -> Self {
        SourceFile {
            inner,
            _marker: MARKER,
        }
    }
    pub fn path(&self) -> PathBuf {}
    pub fn is_real(&self) -> bool {}
}
