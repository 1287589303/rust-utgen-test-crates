fn len(&self) -> usize {
        self.table().len() >> self.stride2
    }