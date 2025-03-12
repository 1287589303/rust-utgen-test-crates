pub fn extract_bytes<'h, const N: usize>(
        &self,
        haystack: &'h [u8],
    ) -> (&'h [u8], [&'h [u8]; N]) {
        let mut matched = self.iter().flatten();
        let whole_match = &haystack[matched.next().expect("a match")];
        let group_matches = [0; N].map(|_| {
            let sp = matched.next().expect("too few matching groups");
            &haystack[sp]
        });
        (whole_match, group_matches)
    }