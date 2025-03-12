fn try_search_half_start(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryError> {
        let mut span = input.get_span();
        let mut min_start = 0;
        loop {
            let litmatch = match self.pre.find(input.haystack(), span) {
                None => return Ok(None),
                Some(span) => span,
            };
            trace!("reverse suffix scan found suffix match at {:?}", litmatch);
            let revinput = input
                .clone()
                .anchored(Anchored::Yes)
                .span(input.start()..litmatch.end);
            match self
                .try_search_half_rev_limited(cache, &revinput, min_start)?
            {
                None => {
                    if span.start >= span.end {
                        break;
                    }
                    span.start = litmatch.start.checked_add(1).unwrap();
                }
                Some(hm) => return Ok(Some(hm)),
            }
            min_start = litmatch.end;
        }
        Ok(None)
    }