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
#[derive(Copy, Clone)]
pub struct Span {
    inner: imp::Span,
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
    pub fn open(&self) -> Span {
        match &self.inner {
            #[cfg(wrap_proc_macro)]
            DelimSpanEnum::Compiler { open, .. } => {
                Span::_new(imp::Span::Compiler(*open))
            }
            DelimSpanEnum::Fallback(span) => Span::_new_fallback(span.first_byte()),
        }
    }
    pub fn close(&self) -> Span {}
}
impl Span {
    fn _new(inner: imp::Span) -> Self {
        Span { inner, _marker: MARKER }
    }
    fn _new_fallback(inner: fallback::Span) -> Self {
        Span {
            inner: imp::Span::from(inner),
            _marker: MARKER,
        }
    }
    pub fn call_site() -> Self {
        Span::_new(imp::Span::call_site())
    }
    pub fn mixed_site() -> Self {
        Span::_new(imp::Span::mixed_site())
    }
    #[cfg(procmacro2_semver_exempt)]
    #[cfg_attr(docsrs, doc(cfg(procmacro2_semver_exempt)))]
    pub fn def_site() -> Self {
        Span::_new(imp::Span::def_site())
    }
    pub fn resolved_at(&self, other: Span) -> Span {}
    pub fn located_at(&self, other: Span) -> Span {}
    #[cfg(wrap_proc_macro)]
    pub fn unwrap(self) -> proc_macro::Span {}
    #[cfg(wrap_proc_macro)]
    pub fn unstable(self) -> proc_macro::Span {}
    #[cfg(all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)))]
    #[cfg_attr(docsrs, doc(cfg(procmacro2_semver_exempt)))]
    pub fn source_file(&self) -> SourceFile {}
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn byte_range(&self) -> Range<usize> {}
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn start(&self) -> LineColumn {}
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn end(&self) -> LineColumn {}
    pub fn join(&self, other: Span) -> Option<Span> {}
    #[cfg(procmacro2_semver_exempt)]
    #[cfg_attr(docsrs, doc(cfg(procmacro2_semver_exempt)))]
    pub fn eq(&self, other: &Span) -> bool {}
    pub fn source_text(&self) -> Option<String> {}
}
