pub fn try_find<'h, I: Into<Input<'h>>>(
        &self,
        cache: &mut Cache,
        input: I,
    ) -> Result<Option<Match>, MatchError> {
        let input = input.into();
        if self.get_nfa().pattern_len() == 1 {
            let mut slots = [None, None];
            let pid = match self.try_search_slots(cache, &input, &mut slots)? {
                None => return Ok(None),
                Some(pid) => pid,
            };
            let start = match slots[0] {
                None => return Ok(None),
                Some(s) => s.get(),
            };
            let end = match slots[1] {
                None => return Ok(None),
                Some(s) => s.get(),
            };
            return Ok(Some(Match::new(pid, Span { start, end })));
        }
        let ginfo = self.get_nfa().group_info();
        let slots_len = ginfo.implicit_slot_len();
        let mut slots = vec![None; slots_len];
        let pid = match self.try_search_slots(cache, &input, &mut slots)? {
            None => return Ok(None),
            Some(pid) => pid,
        };
        let start = match slots[pid.as_usize() * 2] {
            None => return Ok(None),
            Some(s) => s.get(),
        };
        let end = match slots[pid.as_usize() * 2 + 1] {
            None => return Ok(None),
            Some(s) => s.get(),
        };
        Ok(Some(Match::new(pid, Span { start, end })))
    }