fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::Compiler(e) => Display::fmt(e, f),
            LexError::Fallback(e) => Display::fmt(e, f),
            LexError::CompilerPanic => {
                let fallback = fallback::LexError::call_site();
                Display::fmt(&fallback, f)
            }
        }
    }