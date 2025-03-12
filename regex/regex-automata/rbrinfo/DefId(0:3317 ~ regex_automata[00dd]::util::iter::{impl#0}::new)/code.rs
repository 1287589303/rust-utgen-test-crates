pub fn new(input: Input<'h>) -> Searcher<'h> {
        Searcher { input, last_match_end: None }
    }