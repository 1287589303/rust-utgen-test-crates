fn fragment_only(mut self, base_url: &Url, mut input: Input<'_>) -> ParseResult<Url> {
        let before_fragment = match base_url.fragment_start {
            Some(i) => base_url.slice(..i),
            None => &*base_url.serialization,
        };
        debug_assert!(self.serialization.is_empty());
        self.serialization
            .reserve(before_fragment.len() + input.chars.as_str().len());
        self.serialization.push_str(before_fragment);
        self.serialization.push('#');
        let next = input.next();
        debug_assert!(next == Some('#'));
        self.parse_fragment(input);
        Ok(Url {
            serialization: self.serialization,
            fragment_start: Some(to_u32(before_fragment.len())?),
            ..*base_url
        })
    }