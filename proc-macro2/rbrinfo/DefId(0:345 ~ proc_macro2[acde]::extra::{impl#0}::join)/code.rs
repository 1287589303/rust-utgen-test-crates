pub fn join(&self) -> Span {
        match &self.inner {
            #[cfg(wrap_proc_macro)]
            DelimSpanEnum::Compiler { join, .. } => Span::_new(imp::Span::Compiler(*join)),
            DelimSpanEnum::Fallback(span) => Span::_new_fallback(*span),
        }
    }