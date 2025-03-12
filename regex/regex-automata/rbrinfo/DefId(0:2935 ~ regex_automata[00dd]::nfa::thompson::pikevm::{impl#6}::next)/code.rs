fn next(&mut self) -> Option<Match> {
        // Splitting 'self' apart seems necessary to appease borrowck.
        let FindMatches { re, ref mut cache, ref mut caps, ref mut it } =
            *self;
        // 'advance' converts errors into panics, which is OK here because
        // the PikeVM can never return an error.
        it.advance(|input| {
            re.search(cache, input, caps);
            Ok(caps.get_match())
        })
    }