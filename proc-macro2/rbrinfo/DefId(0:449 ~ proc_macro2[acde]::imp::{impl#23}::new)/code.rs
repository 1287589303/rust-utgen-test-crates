pub(crate) fn new(delimiter: Delimiter, stream: TokenStream) -> Self {
        match stream {
            TokenStream::Compiler(tts) => {
                let delimiter = match delimiter {
                    Delimiter::Parenthesis => proc_macro::Delimiter::Parenthesis,
                    Delimiter::Bracket => proc_macro::Delimiter::Bracket,
                    Delimiter::Brace => proc_macro::Delimiter::Brace,
                    Delimiter::None => proc_macro::Delimiter::None,
                };
                Group::Compiler(proc_macro::Group::new(delimiter, tts.into_token_stream()))
            }
            TokenStream::Fallback(stream) => {
                Group::Fallback(fallback::Group::new(delimiter, stream))
            }
        }
    }