use crate::fallback;
use crate::imp;
use crate::marker::{ProcMacroAutoTraits, MARKER};
use crate::Span;
use core::fmt::{self, Debug};
#[derive(Copy, Clone)]
pub struct DelimSpan {
    inner: DelimSpanEnum,
    _marker: ProcMacroAutoTraits,
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
#[cfg_attr(
    all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)),
    derive(PartialEq, Eq)
)]
pub(crate) struct ProcMacroAutoTraits(PhantomData<Rc<()>>);
#[derive(Copy, Clone)]
pub(crate) enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}
#[derive(Copy, Clone)]
enum DelimSpanEnum {
    #[cfg(wrap_proc_macro)]
    Compiler { join: proc_macro::Span, open: proc_macro::Span, close: proc_macro::Span },
    Fallback(fallback::Span),
}
impl Debug for DelimSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.join(), f)
    }
}
impl DelimSpan {
    pub(crate) fn new(group: &imp::Group) -> Self {
        #[cfg(wrap_proc_macro)]
        let inner = match group {
            imp::Group::Compiler(group) => {
                DelimSpanEnum::Compiler {
                    join: group.span(),
                    open: group.span_open(),
                    close: group.span_close(),
                }
            }
            imp::Group::Fallback(group) => DelimSpanEnum::Fallback(group.span()),
        };
        #[cfg(not(wrap_proc_macro))]
        let inner = DelimSpanEnum::Fallback(group.span());
        DelimSpan {
            inner,
            _marker: MARKER,
        }
    }
    pub fn join(&self) -> Span {
        match &self.inner {
            #[cfg(wrap_proc_macro)]
            DelimSpanEnum::Compiler { join, .. } => {
                Span::_new(imp::Span::Compiler(*join))
            }
            DelimSpanEnum::Fallback(span) => Span::_new_fallback(*span),
        }
    }
    pub fn open(&self) -> Span {}
    pub fn close(&self) -> Span {}
}
