pub(crate) fn build(self) -> TokenStream {
        TokenStream {
            inner: self.inner.build(),
        }
    }