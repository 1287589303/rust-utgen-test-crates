fn try_search_fwd(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).try_search_fwd(input)
    }