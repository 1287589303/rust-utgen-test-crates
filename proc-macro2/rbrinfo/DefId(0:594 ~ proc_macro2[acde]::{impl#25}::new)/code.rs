pub fn new(delimiter: Delimiter, stream: TokenStream) -> Self {
        Group {
            inner: imp::Group::new(delimiter, stream.inner),
        }
    }