fn try_search_full(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Option<Match>, RetryError> {
        let mut span = input.get_span();
        let mut min_match_start = 0;
        let mut min_pre_start = 0;
        loop {
            let litmatch = match self.preinner.find(input.haystack(), span) {
                None => return Ok(None),
                Some(span) => span,
            };
            if litmatch.start < min_pre_start {
                trace!(
                    "found inner prefilter match at {:?}, which starts \
					 before the end of the last forward scan at {}, \
					 quitting to avoid quadratic behavior",
                    litmatch,
                    min_pre_start,
                );
                return Err(RetryError::Quadratic(RetryQuadraticError::new()));
            }
            trace!("reverse inner scan found inner match at {:?}", litmatch);
            let revinput = input
                .clone()
                .anchored(Anchored::Yes)
                .span(input.start()..litmatch.start);
            // Note that in addition to the literal search above scanning past
            // our minimum start point, this routine can also return an error
            // as a result of detecting possible quadratic behavior if the
            // reverse scan goes past the minimum start point. That is, the
            // literal search might not, but the reverse regex search for the
            // prefix might!
            match self.try_search_half_rev_limited(
                cache,
                &revinput,
                min_match_start,
            )? {
                None => {
                    if span.start >= span.end {
                        break;
                    }
                    span.start = litmatch.start.checked_add(1).unwrap();
                }
                Some(hm_start) => {
                    let fwdinput = input
                        .clone()
                        .anchored(Anchored::Pattern(hm_start.pattern()))
                        .span(hm_start.offset()..input.end());
                    match self.try_search_half_fwd_stopat(cache, &fwdinput)? {
                        Err(stopat) => {
                            min_pre_start = stopat;
                            span.start =
                                litmatch.start.checked_add(1).unwrap();
                        }
                        Ok(hm_end) => {
                            return Ok(Some(Match::new(
                                hm_start.pattern(),
                                hm_start.offset()..hm_end.offset(),
                            )))
                        }
                    }
                }
            }
            min_match_start = litmatch.end;
        }
        Ok(None)
    }