pub fn is_match<'h, I: Into<Input<'h>>>(
        &self,
        cache: &mut Cache,
        input: I,
    ) -> bool {
        let mut input = input.into().earliest(true);
        if matches!(input.get_anchored(), Anchored::No) {
            input.set_anchored(Anchored::Yes);
        }
        self.try_search_slots(cache, &input, &mut []).unwrap().is_some()
    }