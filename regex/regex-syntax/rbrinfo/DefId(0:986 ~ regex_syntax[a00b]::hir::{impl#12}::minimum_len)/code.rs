pub fn minimum_len(&self) -> Option<usize> {
        let first = self.ranges().get(0)?;
        // Correct because c1 < c2 implies c1.len_utf8() < c2.len_utf8().
        Some(first.start.len_utf8())
    }