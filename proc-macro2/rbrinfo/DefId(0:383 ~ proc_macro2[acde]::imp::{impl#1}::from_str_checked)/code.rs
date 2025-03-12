pub(crate) fn from_str_checked(src: &str) -> Result<Self, LexError> {
        if inside_proc_macro() {
            Ok(TokenStream::Compiler(DeferredTokenStream::new(
                proc_macro::TokenStream::from_str_checked(src)?,
            )))
        } else {
            Ok(TokenStream::Fallback(
                fallback::TokenStream::from_str_checked(src)?,
            ))
        }
    }