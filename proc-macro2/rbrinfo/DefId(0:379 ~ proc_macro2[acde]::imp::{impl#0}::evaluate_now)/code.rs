fn evaluate_now(&mut self) {
        // If-check provides a fast short circuit for the common case of `extra`
        // being empty, which saves a round trip over the proc macro bridge.
        // Improves macro expansion time in winrt by 6% in debug mode.
        if !self.extra.is_empty() {
            self.stream.extend(self.extra.drain(..));
        }
    }