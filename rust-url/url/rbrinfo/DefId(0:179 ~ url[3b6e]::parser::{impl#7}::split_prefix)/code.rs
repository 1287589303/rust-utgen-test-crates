fn split_prefix(self, input: &mut Input) -> bool {
        for c in self.chars() {
            if input.next() != Some(c) {
                return false;
            }
        }
        true
    }