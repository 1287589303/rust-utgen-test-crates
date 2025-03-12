fn split_prefix(self, input: &mut Input) -> bool {
        input.next().map_or(false, self)
    }