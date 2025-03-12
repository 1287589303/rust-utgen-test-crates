pub fn maximum_len(&self) -> Option<usize> {
        let last = self.ranges().last()?;
        // Correct because c1 < c2 implies c1.len_utf8() < c2.len_utf8().
        Some(last.end.len_utf8())
    }