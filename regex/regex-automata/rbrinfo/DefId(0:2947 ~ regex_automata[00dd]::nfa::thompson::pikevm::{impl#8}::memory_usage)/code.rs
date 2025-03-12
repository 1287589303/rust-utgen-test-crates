pub fn memory_usage(&self) -> usize {
        use core::mem::size_of;
        (self.stack.len() * size_of::<FollowEpsilon>())
            + self.curr.memory_usage()
            + self.next.memory_usage()
    }