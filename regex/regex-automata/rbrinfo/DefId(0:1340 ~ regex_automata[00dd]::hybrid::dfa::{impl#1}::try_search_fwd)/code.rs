pub fn try_search_fwd(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, MatchError> {
        let utf8empty = self.get_nfa().has_empty() && self.get_nfa().is_utf8();
        let hm = match search::find_fwd(self, cache, input)? {
            None => return Ok(None),
            Some(hm) if !utf8empty => return Ok(Some(hm)),
            Some(hm) => hm,
        };
        // We get to this point when we know our DFA can match the empty string
        // AND when UTF-8 mode is enabled. In this case, we skip any matches
        // whose offset splits a codepoint. Such a match is necessarily a
        // zero-width match, because UTF-8 mode requires the underlying NFA
        // to be built such that all non-empty matches span valid UTF-8.
        // Therefore, any match that ends in the middle of a codepoint cannot
        // be part of a span of valid UTF-8 and thus must be an empty match.
        // In such cases, we skip it, so as not to report matches that split a
        // codepoint.
        //
        // Note that this is not a checked assumption. Callers *can* provide an
        // NFA with UTF-8 mode enabled but produces non-empty matches that span
        // invalid UTF-8. But doing so is documented to result in unspecified
        // behavior.
        empty::skip_splits_fwd(input, hm, hm.offset(), |input| {
            let got = search::find_fwd(self, cache, input)?;
            Ok(got.map(|hm| (hm, hm.offset())))
        })
    }