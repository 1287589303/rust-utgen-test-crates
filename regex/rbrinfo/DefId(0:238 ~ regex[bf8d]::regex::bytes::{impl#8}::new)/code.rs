fn new(haystack: &'h [u8], start: usize, end: usize) -> Match<'h> {
        Match { haystack, start, end }
    }