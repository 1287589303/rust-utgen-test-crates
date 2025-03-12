fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::Compiler(e) => Debug::fmt(e, f),
            LexError::Fallback(e) => Debug::fmt(e, f),
            LexError::CompilerPanic => {
                let fallback = fallback::LexError::call_site();
                Debug::fmt(&fallback, f)
            }
        }
    }