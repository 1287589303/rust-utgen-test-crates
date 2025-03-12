fn split_prefix(self, input: &mut Input) -> bool {
        input.next() == Some(self)
    }