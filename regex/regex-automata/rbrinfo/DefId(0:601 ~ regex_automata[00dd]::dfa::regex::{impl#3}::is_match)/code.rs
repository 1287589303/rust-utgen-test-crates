pub fn is_match<'h, I: Into<Input<'h>>>(&self, input: I) -> bool {
        // Not only can we do an "earliest" search, but we can avoid doing a
        // reverse scan too.
        let input = input.into().earliest(true);
        self.forward().try_search_fwd(&input).map(|x| x.is_some()).unwrap()
    }