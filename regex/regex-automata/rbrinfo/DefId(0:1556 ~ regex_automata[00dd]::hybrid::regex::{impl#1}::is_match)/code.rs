pub fn is_match<'h, I: Into<Input<'h>>>(
        &self,
        cache: &mut Cache,
        input: I,
    ) -> bool {
        // Not only can we do an "earliest" search, but we can avoid doing a
        // reverse scan too.
        self.forward()
            .try_search_fwd(&mut cache.forward, &input.into().earliest(true))
            .unwrap()
            .is_some()
    }