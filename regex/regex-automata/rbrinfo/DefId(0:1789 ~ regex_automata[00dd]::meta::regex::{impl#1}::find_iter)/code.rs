pub fn find_iter<'r, 'h, I: Into<Input<'h>>>(
        &'r self,
        input: I,
    ) -> FindMatches<'r, 'h> {
        let cache = self.pool.get();
        let it = iter::Searcher::new(input.into());
        FindMatches { re: self, cache, it }
    }