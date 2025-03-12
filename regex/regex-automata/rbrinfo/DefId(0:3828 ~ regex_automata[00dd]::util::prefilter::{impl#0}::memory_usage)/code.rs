pub fn memory_usage(&self) -> usize {
        #[cfg(not(feature = "alloc"))]
        {
            unreachable!()
        }
        #[cfg(feature = "alloc")]
        {
            self.pre.memory_usage()
        }
    }