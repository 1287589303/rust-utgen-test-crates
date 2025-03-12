fn active_chunk_start(&self) -> usize {
        self.chunks.last().map_or(0, |&(_, end)| end)
    }