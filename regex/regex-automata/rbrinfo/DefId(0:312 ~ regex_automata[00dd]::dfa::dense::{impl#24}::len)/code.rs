fn len(&self) -> usize {
        assert_eq!(0, self.slices().len() % 2);
        self.slices().len() / 2
    }