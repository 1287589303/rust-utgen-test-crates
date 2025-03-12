fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self {
        let mut streams = streams.into_iter();
        match streams.next() {
            Some(TokenStream::Compiler(mut first)) => {
                first.evaluate_now();
                first.stream.extend(streams.map(|s| match s {
                    TokenStream::Compiler(s) => s.into_token_stream(),
                    TokenStream::Fallback(_) => mismatch(line!()),
                }));
                TokenStream::Compiler(first)
            }
            Some(TokenStream::Fallback(mut first)) => {
                first.extend(streams.map(|s| match s {
                    TokenStream::Fallback(s) => s,
                    TokenStream::Compiler(_) => mismatch(line!()),
                }));
                TokenStream::Fallback(first)
            }
            None => TokenStream::new(),
        }
    }