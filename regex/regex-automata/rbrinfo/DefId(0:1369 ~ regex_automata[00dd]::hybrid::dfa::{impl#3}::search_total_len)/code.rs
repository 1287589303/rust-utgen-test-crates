pub fn search_total_len(&self) -> usize {
        self.bytes_searched + self.progress.as_ref().map_or(0, |p| p.len())
    }