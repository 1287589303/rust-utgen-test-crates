fn next(&mut self) -> Option<Captures<'h>> {
        let static_captures_len = self.it.regex().static_captures_len();
        self.it.next().map(|caps| Captures {
            haystack: self.haystack,
            caps,
            static_captures_len,
        })
    }