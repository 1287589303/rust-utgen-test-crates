fn is_intersection_empty(&self, other: &ClassRange) -> bool {
        let (s1, e1) = (self.start, self.end);
        let (s2, e2) = (other.start, other.end);
        core::cmp::max(s1, s2) > core::cmp::min(e1, e2)
    }