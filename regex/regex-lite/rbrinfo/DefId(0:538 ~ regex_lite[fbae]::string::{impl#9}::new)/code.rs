fn new(haystack: &'h str, start: usize, end: usize) -> Match<'h> {
        Match { haystack, start, end }
    }