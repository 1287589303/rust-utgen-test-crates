fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, stream: I) {
        match self {
            TokenStream::Compiler(tts) => {
                // Here is the reason for DeferredTokenStream.
                for token in stream {
                    tts.extra.push(into_compiler_token(token));
                }
            }
            TokenStream::Fallback(tts) => tts.extend(stream),
        }
    }