fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenStream::Compiler(tts) => Debug::fmt(&tts.clone().into_token_stream(), f),
            TokenStream::Fallback(tts) => Debug::fmt(tts, f),
        }
    }