fn write_to_len(&self) -> usize {
        size_of::<u32>()   // state length
        + size_of::<u32>() // pattern length
        + self.classes.write_to_len()
        + size_of::<u32>() // sparse transitions length
        + self.sparse().len()
    }