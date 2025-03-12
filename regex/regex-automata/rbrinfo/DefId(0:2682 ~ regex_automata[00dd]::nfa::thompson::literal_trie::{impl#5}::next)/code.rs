fn next(&mut self) -> Option<&'a [Transition]> {
        if let Some(&(start, end)) = self.chunks.next() {
            return Some(&self.transitions[start..end]);
        }
        if let Some(chunk) = self.active.take() {
            return Some(chunk);
        }
        None
    }