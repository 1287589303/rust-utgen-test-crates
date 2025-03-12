pub const fn new() -> OnceBool {
        OnceBool { inner: OnceNonZeroUsize::new() }
    }