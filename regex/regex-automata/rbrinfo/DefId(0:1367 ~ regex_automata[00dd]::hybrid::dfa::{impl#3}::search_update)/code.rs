pub fn search_update(&mut self, at: usize) {
        let p =
            self.progress.as_mut().expect("no in-progress search to update");
        p.at = at;
    }