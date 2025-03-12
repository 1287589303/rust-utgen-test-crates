pub(crate) fn push_token_from_parser(&mut self, tt: TokenTree) {
        self.inner.push(tt);
    }