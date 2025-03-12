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
enum DelimSpanEnum {
    #[cfg(wrap_proc_macro)]
    Compiler { join: proc_macro::Span, open: proc_macro::Span, close: proc_macro::Span },
    Fallback(fallback::Span),
}
#[derive(Copy, Clone)]
pub(crate) enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
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
    pub fn join(&self) -> Span {}
    pub fn open(&self) -> Span {}
    pub fn close(&self) -> Span {
        match &self.inner {
            #[cfg(wrap_proc_macro)]
            DelimSpanEnum::Compiler { close, .. } => {
                Span::_new(imp::Span::Compiler(*close))
            }
            DelimSpanEnum::Fallback(span) => Span::_new_fallback(span.last_byte()),
        }
    }
}
impl Span {
    #[cfg(not(span_locations))]
    pub(crate) fn call_site() -> Self {
        Span {}
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
    #[cfg(span_locations)]
    pub(crate) fn last_byte(self) -> Self {
        Span {
            lo: cmp::max(self.hi.saturating_sub(1), self.lo),
            hi: self.hi,
        }
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
