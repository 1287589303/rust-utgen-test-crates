pub fn max_needle_len(&self) -> usize {
        #[cfg(not(feature = "alloc"))]
        {
            unreachable!()
        }
        #[cfg(feature = "alloc")]
        {
            self.max_needle_len
        }
    }