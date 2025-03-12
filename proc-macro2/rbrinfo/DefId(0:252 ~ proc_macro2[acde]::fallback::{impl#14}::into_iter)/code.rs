fn into_iter(self) -> TokenTreeIter {
        self.take_inner().into_iter()
    }