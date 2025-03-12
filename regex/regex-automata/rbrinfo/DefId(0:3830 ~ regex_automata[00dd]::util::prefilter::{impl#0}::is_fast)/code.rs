pub fn is_fast(&self) -> bool {
        #[cfg(not(feature = "alloc"))]
        {
            unreachable!()
        }
        #[cfg(feature = "alloc")]
        {
            self.is_fast
        }
    }