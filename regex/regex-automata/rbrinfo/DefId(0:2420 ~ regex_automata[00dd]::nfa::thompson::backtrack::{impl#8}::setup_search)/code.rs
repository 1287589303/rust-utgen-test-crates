fn setup_search(
        &mut self,
        re: &BoundedBacktracker,
        input: &Input<'_>,
    ) -> Result<(), MatchError> {
        // Our haystack length is only the length of the span of the entire
        // haystack that we'll be searching.
        let haylen = input.get_span().len();
        let err = || MatchError::haystack_too_long(haylen);
        // Our stride is one more than the length of the input because our main
        // search loop includes the position at input.end(). (And it does this
        // because matches are delayed by one byte to account for look-around.)
        self.stride = haylen + 1;
        let needed_capacity =
            match re.get_nfa().states().len().checked_mul(self.stride) {
                None => return Err(err()),
                Some(capacity) => capacity,
            };
        let max_capacity = 8 * re.get_config().get_visited_capacity();
        if needed_capacity > max_capacity {
            return Err(err());
        }
        let needed_blocks = div_ceil(needed_capacity, Visited::BLOCK_SIZE);
        self.bitset.truncate(needed_blocks);
        for block in self.bitset.iter_mut() {
            *block = 0;
        }
        if needed_blocks > self.bitset.len() {
            self.bitset.resize(needed_blocks, 0);
        }
        Ok(())
    }