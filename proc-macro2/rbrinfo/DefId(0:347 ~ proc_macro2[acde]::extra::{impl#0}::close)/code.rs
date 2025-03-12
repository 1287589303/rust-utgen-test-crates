pub fn close(&self) -> Span {
        match &self.inner {
            #[cfg(wrap_proc_macro)]
            DelimSpanEnum::Compiler { close, .. } => Span::_new(imp::Span::Compiler(*close)),
            DelimSpanEnum::Fallback(span) => Span::_new_fallback(span.last_byte()),
        }
    }