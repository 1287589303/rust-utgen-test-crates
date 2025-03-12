fn to_owned(&self) -> StartTable<alloc::vec::Vec<u32>> {
        StartTable {
            table: self.table.as_ref().to_vec(),
            kind: self.kind,
            start_map: self.start_map.clone(),
            stride: self.stride,
            pattern_len: self.pattern_len,
            universal_start_unanchored: self.universal_start_unanchored,
            universal_start_anchored: self.universal_start_anchored,
        }
    }