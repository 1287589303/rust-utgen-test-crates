pub fn new() -> Builder {
        Builder {
            #[cfg(feature = "dfa-build")]
            dfa: dense::Builder::new(),
        }
    }