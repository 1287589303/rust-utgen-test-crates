fn into_iter(self) -> TokenTreeIter {
        match self {
            TokenStream::Compiler(tts) => {
                TokenTreeIter::Compiler(tts.into_token_stream().into_iter())
            }
            TokenStream::Fallback(tts) => TokenTreeIter::Fallback(tts.into_iter()),
        }
    }