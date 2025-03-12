pub fn search_start(&mut self, at: usize) {
        // If a previous search wasn't marked as finished, then finish it
        // now automatically.
        if let Some(p) = self.progress.take() {
            self.bytes_searched += p.len();
        }
        self.progress = Some(SearchProgress { start: at, at });
    }