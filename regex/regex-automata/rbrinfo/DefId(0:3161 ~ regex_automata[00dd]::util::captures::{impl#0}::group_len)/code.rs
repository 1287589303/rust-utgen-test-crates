pub fn group_len(&self) -> usize {
        let pid = match self.pattern() {
            None => return 0,
            Some(pid) => pid,
        };
        self.group_info().group_len(pid)
    }