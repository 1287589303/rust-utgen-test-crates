fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self {
        TokenStream::_new(streams.into_iter().map(|i| i.inner).collect())
    }