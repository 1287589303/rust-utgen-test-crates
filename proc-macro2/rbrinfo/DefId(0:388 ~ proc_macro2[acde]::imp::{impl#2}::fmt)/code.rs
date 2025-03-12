fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenStream::Compiler(tts) => Display::fmt(&tts.clone().into_token_stream(), f),
            TokenStream::Fallback(tts) => Display::fmt(tts, f),
        }
    }