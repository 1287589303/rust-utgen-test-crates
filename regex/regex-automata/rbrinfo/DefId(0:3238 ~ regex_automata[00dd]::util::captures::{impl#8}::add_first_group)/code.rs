fn add_first_group(&mut self, pid: PatternID) {
        assert_eq!(pid.as_usize(), self.slot_ranges.len());
        assert_eq!(pid.as_usize(), self.name_to_index.len());
        assert_eq!(pid.as_usize(), self.index_to_name.len());
        // This is the start of our slots for the explicit capturing groups.
        // Note that since the slots for the 0th group for every pattern appear
        // before any slots for the nth group (where n > 0) in any pattern, we
        // will have to fix up the slot ranges once we know how many patterns
        // we've added capture groups for.
        let slot_start = self.small_slot_len();
        self.slot_ranges.push((slot_start, slot_start));
        self.name_to_index.push(CaptureNameMap::new());
        self.index_to_name.push(vec![None]);
        self.memory_extra += core::mem::size_of::<Option<Arc<str>>>();
    }