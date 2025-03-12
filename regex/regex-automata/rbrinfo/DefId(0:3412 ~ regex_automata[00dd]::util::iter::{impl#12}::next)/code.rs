fn next(&mut self) -> Option<Result<Captures, MatchError>> {
        let TryCapturesIter { ref mut it, ref mut caps, ref mut finder } =
            *self;
        let result = it
            .try_advance(|input| {
                (finder)(input, caps)?;
                Ok(caps.get_match())
            })
            .transpose()?;
        match result {
            Ok(_) => Some(Ok(caps.clone())),
            Err(err) => Some(Err(err)),
        }
    }