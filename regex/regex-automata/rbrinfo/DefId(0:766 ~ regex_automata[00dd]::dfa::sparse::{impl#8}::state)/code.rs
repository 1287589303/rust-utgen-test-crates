fn state(&self, id: StateID) -> State<'_> {
        let mut state = &self.sparse()[id.as_usize()..];
        let mut ntrans = wire::read_u16(&state).as_usize();
        let is_match = (1 << 15) & ntrans != 0;
        ntrans &= !(1 << 15);
        state = &state[2..];

        let (input_ranges, state) = state.split_at(ntrans * 2);
        let (next, state) = state.split_at(ntrans * StateID::SIZE);
        let (pattern_ids, state) = if is_match {
            let npats = wire::read_u32(&state).as_usize();
            state[4..].split_at(npats * 4)
        } else {
            (&[][..], state)
        };

        let accel_len = usize::from(state[0]);
        let accel = &state[1..accel_len + 1];
        State { id, is_match, ntrans, input_ranges, next, pattern_ids, accel }
    }