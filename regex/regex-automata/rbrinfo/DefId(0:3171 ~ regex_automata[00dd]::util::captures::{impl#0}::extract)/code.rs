pub fn extract<'h, const N: usize>(
        &self,
        haystack: &'h str,
    ) -> (&'h str, [&'h str; N]) {
        let mut matched = self.iter().flatten();
        let whole_match = &haystack[matched.next().expect("a match")];
        let group_matches = [0; N].map(|_| {
            let sp = matched.next().expect("too few matching groups");
            &haystack[sp]
        });
        (whole_match, group_matches)
    }