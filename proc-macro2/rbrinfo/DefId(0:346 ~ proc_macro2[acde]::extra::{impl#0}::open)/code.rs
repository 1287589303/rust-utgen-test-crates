pub fn open(&self) -> Span {
        match &self.inner {
            #[cfg(wrap_proc_macro)]
            DelimSpanEnum::Compiler { open, .. } => Span::_new(imp::Span::Compiler(*open)),
            DelimSpanEnum::Fallback(span) => Span::_new_fallback(span.first_byte()),
        }
    }