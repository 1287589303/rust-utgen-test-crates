fn write_to_len(&self) -> usize {
        size_of::<u32>()   // state length
        + size_of::<u32>() // stride2
        + self.classes.write_to_len()
        + (self.table().len() * StateID::SIZE)
    }