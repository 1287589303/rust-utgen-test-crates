pub fn captures_at<'h>(
        &self,
        haystack: &'h [u8],
        start: usize,
    ) -> Option<Captures<'h>> {
        let input = Input::new(haystack).span(start..haystack.len());
        let mut caps = self.meta.create_captures();
        self.meta.captures(input, &mut caps);
        if caps.is_match() {
            let static_captures_len = self.static_captures_len();
            Some(Captures { haystack, caps, static_captures_len })
        } else {
            None
        }
    }