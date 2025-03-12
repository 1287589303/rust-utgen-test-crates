fn extend<I: IntoIterator<Item = TokenStream>>(&mut self, streams: I) {
        match self {
            TokenStream::Compiler(tts) => {
                tts.evaluate_now();
                tts.stream
                    .extend(streams.into_iter().map(TokenStream::unwrap_nightly));
            }
            TokenStream::Fallback(tts) => {
                tts.extend(streams.into_iter().map(TokenStream::unwrap_stable));
            }
        }
    }