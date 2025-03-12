fn len(&self) -> usize {
        self.table().len() / StateID::SIZE
    }