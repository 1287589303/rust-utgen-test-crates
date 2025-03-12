fn write_to_len(&self) -> usize {
        size_of::<u32>()   // match state length
        + (self.slices().len() * PatternID::SIZE)
        + size_of::<u32>() // unique pattern ID length
        + size_of::<u32>() // pattern ID length
        + (self.pattern_ids().len() * PatternID::SIZE)
    }