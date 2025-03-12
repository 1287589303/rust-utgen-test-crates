fn from_str_checked(src: &str) -> Result<Self, imp::LexError> {
        // Validate using fallback parser, because rustc is incapable of
        // returning a recoverable Err for certain invalid token streams, and
        // will instead permanently poison the compilation.
        if !Self::valid(src) {
            return Err(imp::LexError::CompilerPanic);
        }

        // Catch panic to work around https://github.com/rust-lang/rust/issues/58736.
        match panic::catch_unwind(|| Self::from_str(src)) {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(lex)) => Err(imp::LexError::Compiler(lex)),
            Err(_panic) => Err(imp::LexError::CompilerPanic),
        }
    }