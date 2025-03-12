fn state_mut(&mut self, id: StateID) -> StateMut<'_> {
        let mut state = &mut self.sparse_mut()[id.as_usize()..];
        let mut ntrans = wire::read_u16(&state).as_usize();
        let is_match = (1 << 15) & ntrans != 0;
        ntrans &= !(1 << 15);
        state = &mut state[2..];

        let (input_ranges, state) = state.split_at_mut(ntrans * 2);
        let (next, state) = state.split_at_mut(ntrans * StateID::SIZE);
        let (pattern_ids, state) = if is_match {
            let npats = wire::read_u32(&state).as_usize();
            state[4..].split_at_mut(npats * 4)
        } else {
            (&mut [][..], state)
        };

        let accel_len = usize::from(state[0]);
        let accel = &mut state[1..accel_len + 1];
        StateMut {
            id,
            is_match,
            ntrans,
            input_ranges,
            next,
            pattern_ids,
            accel,
        }
    }