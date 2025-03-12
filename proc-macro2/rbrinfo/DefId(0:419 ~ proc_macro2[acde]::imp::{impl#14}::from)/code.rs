fn from(e: fallback::LexError) -> Self {
        LexError::Fallback(e)
    }