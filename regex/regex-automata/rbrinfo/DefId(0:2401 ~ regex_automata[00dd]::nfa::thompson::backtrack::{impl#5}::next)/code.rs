fn next(&mut self) -> Option<Result<Match, MatchError>> {
        // Splitting 'self' apart seems necessary to appease borrowck.
        let TryFindMatches { re, ref mut cache, ref mut caps, ref mut it } =
            *self;
        it.try_advance(|input| {
            re.try_search(cache, input, caps)?;
            Ok(caps.get_match())
        })
        .transpose()
    }