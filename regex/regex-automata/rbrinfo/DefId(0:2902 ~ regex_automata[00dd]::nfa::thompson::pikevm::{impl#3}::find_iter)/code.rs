pub fn find_iter<'r, 'c, 'h, I: Into<Input<'h>>>(
        &'r self,
        cache: &'c mut Cache,
        input: I,
    ) -> FindMatches<'r, 'c, 'h> {
        let caps = Captures::matches(self.get_nfa().group_info().clone());
        let it = iter::Searcher::new(input.into());
        FindMatches { re: self, cache, caps, it }
    }