pub fn captures<'h, I: Into<Input<'h>>>(
        &self,
        input: I,
        caps: &mut Captures,
    ) {
        self.search_captures(&input.into(), caps)
    }