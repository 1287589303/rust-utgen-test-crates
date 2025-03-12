fn small_slot_len(&self) -> SmallIndex {
        // Since slots are allocated in order of pattern (starting at 0) and
        // then in order of capture group, it follows that the number of slots
        // is the end of the range of slots for the last pattern. This is
        // true even when the last pattern has no capturing groups, since
        // 'slot_ranges' will still represent it explicitly with an empty
        // range.
        self.slot_ranges.last().map_or(SmallIndex::ZERO, |&(_, end)| end)
    }