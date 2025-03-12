fn finish(mut self) -> &'a mut Url {
        let url = self.url.take().unwrap();
        url.restore_already_parsed_fragment(self.fragment.take());
        url
    }