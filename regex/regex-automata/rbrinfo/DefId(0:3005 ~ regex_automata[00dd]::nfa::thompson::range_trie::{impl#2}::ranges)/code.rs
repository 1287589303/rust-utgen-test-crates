fn ranges(&self) -> &[Utf8Range] {
        &self.ranges[..usize::try_from(self.len).unwrap()]
    }