fn from(e: proc_macro::LexError) -> Self {
        LexError::Compiler(e)
    }