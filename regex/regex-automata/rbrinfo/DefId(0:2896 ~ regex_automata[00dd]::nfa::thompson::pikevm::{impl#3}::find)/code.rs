pub fn find<'h, I: Into<Input<'h>>>(
        &self,
        cache: &mut Cache,
        input: I,
    ) -> Option<Match> {
        let input = input.into();
        if self.get_nfa().pattern_len() == 1 {
            let mut slots = [None, None];
            let pid = self.search_slots(cache, &input, &mut slots)?;
            let start = slots[0]?.get();
            let end = slots[1]?.get();
            return Some(Match::new(pid, Span { start, end }));
        }
        let ginfo = self.get_nfa().group_info();
        let slots_len = ginfo.implicit_slot_len();
        let mut slots = vec![None; slots_len];
        let pid = self.search_slots(cache, &input, &mut slots)?;
        let start = slots[pid.as_usize() * 2]?.get();
        let end = slots[pid.as_usize() * 2 + 1]?.get();
        Some(Match::new(pid, Span { start, end }))
    }