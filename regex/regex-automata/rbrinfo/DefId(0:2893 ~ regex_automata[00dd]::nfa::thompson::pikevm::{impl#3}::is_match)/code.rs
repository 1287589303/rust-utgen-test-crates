pub fn is_match<'h, I: Into<Input<'h>>>(
        &self,
        cache: &mut Cache,
        input: I,
    ) -> bool {
        let input = input.into().earliest(true);
        self.search_slots(cache, &input, &mut []).is_some()
    }