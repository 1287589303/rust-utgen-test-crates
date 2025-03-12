fn write_to_len(&self) -> usize {
        let mut len = 2
            + (self.ntrans * 2)
            + (self.ntrans * StateID::SIZE)
            + (1 + self.accel.len());
        if self.is_match {
            len += size_of::<u32>() + self.pattern_ids.len();
        }
        len
    }