fn active_chunk(&self) -> &[Transition] {
        let start = self.active_chunk_start();
        &self.transitions[start..]
    }