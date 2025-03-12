fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self {
        let mut v = RcVecBuilder::new();

        for stream in streams {
            v.extend(stream.take_inner());
        }

        TokenStream { inner: v.build() }
    }