pub fn find<'h, I: Into<Input<'h>>>(
        &self,
        cache: &mut Cache,
        input: I,
    ) -> Option<Match> {
        let mut input = input.into();
        if matches!(input.get_anchored(), Anchored::No) {
            input.set_anchored(Anchored::Yes);
        }
        if self.get_nfa().pattern_len() == 1 {
            let mut slots = [None, None];
            let pid =
                self.try_search_slots(cache, &input, &mut slots).unwrap()?;
            let start = slots[0].unwrap().get();
            let end = slots[1].unwrap().get();
            return Some(Match::new(pid, Span { start, end }));
        }
        let ginfo = self.get_nfa().group_info();
        let slots_len = ginfo.implicit_slot_len();
        let mut slots = vec![None; slots_len];
        let pid = self.try_search_slots(cache, &input, &mut slots).unwrap()?;
        let start = slots[pid.as_usize() * 2].unwrap().get();
        let end = slots[pid.as_usize() * 2 + 1].unwrap().get();
        Some(Match::new(pid, Span { start, end }))
    }