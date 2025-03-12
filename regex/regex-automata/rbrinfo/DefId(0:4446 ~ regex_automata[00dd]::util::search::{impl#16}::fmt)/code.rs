fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "failed to insert pattern ID {} into pattern set \
             with insufficiet capacity of {}",
            self.attempted.as_usize(),
            self.capacity,
        )
    }