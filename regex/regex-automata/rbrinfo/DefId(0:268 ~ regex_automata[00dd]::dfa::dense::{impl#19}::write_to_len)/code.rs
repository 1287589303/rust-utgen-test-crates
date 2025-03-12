fn write_to_len(&self) -> usize {
        self.kind.write_to_len()
        + self.start_map.write_to_len()
        + size_of::<u32>() // stride
        + size_of::<u32>() // # patterns
        + size_of::<u32>() // universal unanchored start
        + size_of::<u32>() // universal anchored start
        + (self.table().len() * StateID::SIZE)
    }