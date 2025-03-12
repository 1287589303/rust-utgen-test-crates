pub fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N]) {
        let len = self
            .pikevm
            .nfa()
            .static_explicit_captures_len()
            .expect("number of capture groups can vary in a match");
        assert_eq!(N, len, "asked for {} groups, but must ask for {}", N, len);
        let mut matched = self.iter().flatten();
        let whole_match = matched.next().expect("a match").as_str();
        let group_matches = [0; N].map(|_| {
            matched.next().expect("too few matching groups").as_str()
        });
        (whole_match, group_matches)
    }