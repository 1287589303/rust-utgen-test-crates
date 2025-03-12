pub fn try_search(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Option<Match>, MatchError> {
        let (fcache, rcache) = (&mut cache.forward, &mut cache.reverse);
        let end = match self.forward().try_search_fwd(fcache, input)? {
            None => return Ok(None),
            Some(end) => end,
        };
        // This special cases an empty match at the beginning of the search. If
        // our end matches our start, then since a reverse DFA can't match past
        // the start, it must follow that our starting position is also our end
        // position. So short circuit and skip the reverse search.
        if input.start() == end.offset() {
            return Ok(Some(Match::new(
                end.pattern(),
                end.offset()..end.offset(),
            )));
        }
        // We can also skip the reverse search if we know our search was
        // anchored. This occurs either when the input config is anchored or
        // when we know the regex itself is anchored. In this case, we know the
        // start of the match, if one is found, must be the start of the
        // search.
        if self.is_anchored(input) {
            return Ok(Some(Match::new(
                end.pattern(),
                input.start()..end.offset(),
            )));
        }
        // N.B. I have tentatively convinced myself that it isn't necessary
        // to specify the specific pattern for the reverse search since the
        // reverse search will always find the same pattern to match as the
        // forward search. But I lack a rigorous proof. Why not just provide
        // the pattern anyway? Well, if it is needed, then leaving it out
        // gives us a chance to find a witness. (Also, if we don't need to
        // specify the pattern, then we don't need to build the reverse DFA
        // with 'starts_for_each_pattern' enabled. It doesn't matter too much
        // for the lazy DFA, but does make the overall DFA bigger.)
        //
        // We also need to be careful to disable 'earliest' for the reverse
        // search, since it could be enabled for the forward search. In the
        // reverse case, to satisfy "leftmost" criteria, we need to match as
        // much as we can. We also need to be careful to make the search
        // anchored. We don't want the reverse search to report any matches
        // other than the one beginning at the end of our forward search.
        let revsearch = input
            .clone()
            .span(input.start()..end.offset())
            .anchored(Anchored::Yes)
            .earliest(false);
        let start = self
            .reverse()
            .try_search_rev(rcache, &revsearch)?
            .expect("reverse search must match if forward search does");
        debug_assert_eq!(
            start.pattern(),
            end.pattern(),
            "forward and reverse search must match same pattern",
        );
        debug_assert!(start.offset() <= end.offset());
        Ok(Some(Match::new(end.pattern(), start.offset()..end.offset())))
    }