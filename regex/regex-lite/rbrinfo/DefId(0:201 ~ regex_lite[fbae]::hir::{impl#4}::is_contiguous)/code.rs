fn is_contiguous(&self, other: &ClassRange) -> bool {
        let (s1, e1) = (u32::from(self.start), u32::from(self.end));
        let (s2, e2) = (u32::from(other.start), u32::from(other.end));
        core::cmp::max(s1, s2) <= core::cmp::min(e1, e2).saturating_add(1)
    }