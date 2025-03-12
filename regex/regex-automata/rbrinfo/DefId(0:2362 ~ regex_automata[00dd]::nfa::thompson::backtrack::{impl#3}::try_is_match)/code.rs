pub fn try_is_match<'h, I: Into<Input<'h>>>(
        &self,
        cache: &mut Cache,
        input: I,
    ) -> Result<bool, MatchError> {
        let input = input.into().earliest(true);
        self.try_search_slots(cache, &input, &mut []).map(|pid| pid.is_some())
    }