pub fn try_captures_iter<'r, 'c, 'h, I: Into<Input<'h>>>(
        &'r self,
        cache: &'c mut Cache,
        input: I,
    ) -> TryCapturesMatches<'r, 'c, 'h> {
        let caps = self.create_captures();
        let it = iter::Searcher::new(input.into());
        TryCapturesMatches { re: self, cache, caps, it }
    }