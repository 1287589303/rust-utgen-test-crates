pub fn search_finish(&mut self, at: usize) {
        let mut p =
            self.progress.take().expect("no in-progress search to finish");
        p.at = at;
        self.bytes_searched += p.len();
    }