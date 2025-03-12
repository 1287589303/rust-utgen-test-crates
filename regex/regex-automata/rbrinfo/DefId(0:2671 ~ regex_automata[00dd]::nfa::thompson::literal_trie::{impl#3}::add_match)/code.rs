fn add_match(&mut self) {
        // This is not strictly necessary, but there's no point in recording
        // another match by adding another chunk if the state has no
        // transitions. Note though that we only skip this if we already know
        // this is a match state, which is only true if 'chunks' is not empty.
        // Basically, if we didn't do this, nothing semantically would change,
        // but we'd end up pushing another chunk and potentially triggering an
        // alloc.
        if self.transitions.is_empty() && !self.chunks.is_empty() {
            return;
        }
        let chunk_start = self.active_chunk_start();
        let chunk_end = self.transitions.len();
        self.chunks.push((chunk_start, chunk_end));
    }