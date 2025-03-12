pub fn captures_at<'h>(
        &self,
        haystack: &'h str,
        start: usize,
    ) -> Option<Captures<'h>> {
        let mut caps = Captures {
            haystack,
            slots: self.capture_locations(),
            pikevm: Arc::clone(&self.pikevm),
        };
        let mut cache = self.pool.get();
        let matched = self.pikevm.search(
            &mut cache,
            haystack.as_bytes(),
            start,
            haystack.len(),
            false,
            &mut caps.slots.0,
        );
        if !matched {
            return None;
        }
        Some(caps)
    }