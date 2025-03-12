fn drop(&mut self) {
        if let Some(url) = self.url.take() {
            url.restore_already_parsed_fragment(self.fragment.take())
        }
    }