pub fn stream(&self) -> TokenStream {
        TokenStream::_new(self.inner.stream())
    }