fn next(&mut self) -> Option<Match> {
        let FindMatches { re, ref mut it } = *self;
        it.advance(|input| re.try_search(input))
    }