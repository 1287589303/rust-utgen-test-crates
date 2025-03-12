fn memory_usage(&self) -> usize {
        #[cfg(not(feature = "perf-literal-multisubstring"))]
        {
            unreachable!()
        }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            self.ac.memory_usage()
        }
    }