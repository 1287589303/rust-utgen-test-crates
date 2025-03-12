pub fn max_cross_len(&self, other: &Seq) -> Option<usize> {
        let len1 = self.len()?;
        let len2 = other.len()?;
        Some(len1.saturating_mul(len2))
    }