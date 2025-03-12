pub fn max_haystack_len(&self) -> usize {
        // The capacity given in the config is "bytes of heap memory," but the
        // capacity we use here is "number of bits." So convert the capacity in
        // bytes to the capacity in bits.
        let capacity = 8 * self.get_config().get_visited_capacity();
        let blocks = div_ceil(capacity, Visited::BLOCK_SIZE);
        let real_capacity = blocks.saturating_mul(Visited::BLOCK_SIZE);
        // It's possible for `real_capacity` to be smaller than the number of
        // NFA states for particularly large regexes, so we saturate towards
        // zero.
        (real_capacity / self.nfa.states().len()).saturating_sub(1)
    }