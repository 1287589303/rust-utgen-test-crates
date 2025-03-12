fn extend<I: IntoIterator<Item = TokenStream>>(&mut self, streams: I) {
        self.inner.make_mut().extend(streams.into_iter().flatten());
    }