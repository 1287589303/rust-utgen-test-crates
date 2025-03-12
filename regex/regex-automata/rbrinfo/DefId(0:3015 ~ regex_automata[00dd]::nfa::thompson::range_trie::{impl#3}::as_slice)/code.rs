fn as_slice(&self) -> &[SplitRange] {
        &self.partitions[..self.len]
    }