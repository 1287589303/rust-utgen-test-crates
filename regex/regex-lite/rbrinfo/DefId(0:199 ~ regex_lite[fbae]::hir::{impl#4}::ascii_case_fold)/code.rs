fn ascii_case_fold(&self) -> Option<ClassRange> {
        if !(ClassRange { start: 'a', end: 'z' }).is_intersection_empty(self) {
            let start = core::cmp::max(self.start, 'a');
            let end = core::cmp::min(self.end, 'z');
            return Some(ClassRange {
                start: char::try_from(u32::from(start) - 32).unwrap(),
                end: char::try_from(u32::from(end) - 32).unwrap(),
            });
        }
        if !(ClassRange { start: 'A', end: 'Z' }).is_intersection_empty(self) {
            let start = core::cmp::max(self.start, 'A');
            let end = core::cmp::min(self.end, 'Z');
            return Some(ClassRange {
                start: char::try_from(u32::from(start) + 32).unwrap(),
                end: char::try_from(u32::from(end) + 32).unwrap(),
            });
        }
        None
    }