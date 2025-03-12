pub fn captures<'h, I: Into<Input<'h>>>(
        &self,
        cache: &mut Cache,
        input: I,
        caps: &mut Captures,
    ) {
        let mut input = input.into();
        if matches!(input.get_anchored(), Anchored::No) {
            input.set_anchored(Anchored::Yes);
        }
        self.try_search(cache, &input, caps).unwrap();
    }