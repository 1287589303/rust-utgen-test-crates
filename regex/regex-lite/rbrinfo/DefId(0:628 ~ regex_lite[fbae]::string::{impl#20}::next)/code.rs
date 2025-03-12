fn next(&mut self) -> Option<Captures<'h>> {
        self.it.next().map(|slots| Captures {
            haystack: self.haystack,
            slots: CaptureLocations(slots),
            pikevm: Arc::clone(&self.re.pikevm),
        })
    }