pub fn try_captures<'h, I: Into<Input<'h>>>(
        &self,
        cache: &mut Cache,
        input: I,
        caps: &mut Captures,
    ) -> Result<(), MatchError> {
        self.try_search(cache, &input.into(), caps)
    }