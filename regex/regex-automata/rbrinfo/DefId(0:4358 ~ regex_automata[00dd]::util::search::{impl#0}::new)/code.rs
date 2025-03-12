pub fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h> {
        // Perform only one call to `haystack.as_ref()` to protect from incorrect
        // implementations that return different values from multiple calls.
        // This is important because there's code that relies on `span` not being
        // out of bounds with respect to the stored `haystack`.
        let haystack = haystack.as_ref();
        Input {
            haystack,
            span: Span { start: 0, end: haystack.len() },
            anchored: Anchored::No,
            earliest: false,
        }
    }