fn union(&self, other: &ClassRange) -> Option<ClassRange> {
        if !self.is_contiguous(other) {
            return None;
        }
        let start = core::cmp::min(self.start, other.start);
        let end = core::cmp::max(self.end, other.end);
        Some(ClassRange { start, end })
    }