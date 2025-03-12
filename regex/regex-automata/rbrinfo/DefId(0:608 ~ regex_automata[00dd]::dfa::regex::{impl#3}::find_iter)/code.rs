pub fn find_iter<'r, 'h, I: Into<Input<'h>>>(
        &'r self,
        input: I,
    ) -> FindMatches<'r, 'h, A> {
        let it = iter::Searcher::new(input.into());
        FindMatches { re: self, it }
    }