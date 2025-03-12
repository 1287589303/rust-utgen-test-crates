pub fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N]) {
        let len = self
            .static_captures_len
            .expect("number of capture groups can vary in a match")
            .checked_sub(1)
            .expect("number of groups is always greater than zero");
        assert_eq!(N, len, "asked for {} groups, but must ask for {}", N, len);
        // The regex-automata variant of extract is a bit more permissive.
        // It doesn't require the number of matching capturing groups to be
        // static, and you can even request fewer groups than what's there. So
        // this is guaranteed to never panic because we've asserted above that
        // the user has requested precisely the number of groups that must be
        // present in any match for this regex.
        self.caps.extract(self.haystack)
    }