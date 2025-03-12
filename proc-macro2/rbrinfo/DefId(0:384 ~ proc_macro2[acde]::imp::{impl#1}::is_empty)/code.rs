pub(crate) fn is_empty(&self) -> bool {
        match self {
            TokenStream::Compiler(tts) => tts.is_empty(),
            TokenStream::Fallback(tts) => tts.is_empty(),
        }
    }