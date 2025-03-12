pub(crate) fn new() -> Self {
        TokenStream {
            inner: RcVecBuilder::new().build(),
        }
    }