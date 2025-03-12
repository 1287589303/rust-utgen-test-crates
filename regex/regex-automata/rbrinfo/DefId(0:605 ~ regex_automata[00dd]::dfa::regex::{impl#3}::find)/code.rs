pub fn find<'h, I: Into<Input<'h>>>(&self, input: I) -> Option<Match> {
        self.try_search(&input.into()).unwrap()
    }