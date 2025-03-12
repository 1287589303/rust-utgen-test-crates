fn try_state(
        &self,
        sp: &Special,
        id: StateID,
    ) -> Result<State<'_>, DeserializeError> {
        if id.as_usize() > self.sparse().len() {
            return Err(DeserializeError::generic(
                "invalid caller provided sparse state ID",
            ));
        }
        let mut state = &self.sparse()[id.as_usize()..];
        // Encoding format starts with a u16 that stores the total number of
        // transitions in this state.
        let (mut ntrans, _) =
            wire::try_read_u16_as_usize(state, "state transition length")?;
        let is_match = ((1 << 15) & ntrans) != 0;
        ntrans &= !(1 << 15);
        state = &state[2..];
        if ntrans > 257 || ntrans == 0 {
            return Err(DeserializeError::generic(
                "invalid transition length",
            ));
        }
        if is_match && !sp.is_match_state(id) {
            return Err(DeserializeError::generic(
                "state marked as match but not in match ID range",
            ));
        } else if !is_match && sp.is_match_state(id) {
            return Err(DeserializeError::generic(
                "state in match ID range but not marked as match state",
            ));
        }

        // Each transition has two pieces: an inclusive range of bytes on which
        // it is defined, and the state ID that those bytes transition to. The
        // pairs come first, followed by a corresponding sequence of state IDs.
        let input_ranges_len = ntrans.checked_mul(2).unwrap();
        wire::check_slice_len(state, input_ranges_len, "sparse byte pairs")?;
        let (input_ranges, state) = state.split_at(input_ranges_len);
        // Every range should be of the form A-B, where A<=B.
        for pair in input_ranges.chunks(2) {
            let (start, end) = (pair[0], pair[1]);
            if start > end {
                return Err(DeserializeError::generic("invalid input range"));
            }
        }

        // And now extract the corresponding sequence of state IDs. We leave
        // this sequence as a &[u8] instead of a &[S] because sparse DFAs do
        // not have any alignment requirements.
        let next_len = ntrans
            .checked_mul(self.id_len())
            .expect("state size * #trans should always fit in a usize");
        wire::check_slice_len(state, next_len, "sparse trans state IDs")?;
        let (next, state) = state.split_at(next_len);
        // We can at least verify that every state ID is in bounds.
        for idbytes in next.chunks(self.id_len()) {
            let (id, _) =
                wire::read_state_id(idbytes, "sparse state ID in try_state")?;
            wire::check_slice_len(
                self.sparse(),
                id.as_usize(),
                "invalid sparse state ID",
            )?;
        }

        // If this is a match state, then read the pattern IDs for this state.
        // Pattern IDs is a u32-length prefixed sequence of native endian
        // encoded 32-bit integers.
        let (pattern_ids, state) = if is_match {
            let (npats, nr) =
                wire::try_read_u32_as_usize(state, "pattern ID length")?;
            let state = &state[nr..];
            if npats == 0 {
                return Err(DeserializeError::generic(
                    "state marked as a match, but pattern length is zero",
                ));
            }

            let pattern_ids_len =
                wire::mul(npats, 4, "sparse pattern ID byte length")?;
            wire::check_slice_len(
                state,
                pattern_ids_len,
                "sparse pattern IDs",
            )?;
            let (pattern_ids, state) = state.split_at(pattern_ids_len);
            for patbytes in pattern_ids.chunks(PatternID::SIZE) {
                wire::read_pattern_id(
                    patbytes,
                    "sparse pattern ID in try_state",
                )?;
            }
            (pattern_ids, state)
        } else {
            (&[][..], state)
        };
        if is_match && pattern_ids.is_empty() {
            return Err(DeserializeError::generic(
                "state marked as a match, but has no pattern IDs",
            ));
        }
        if sp.is_match_state(id) && pattern_ids.is_empty() {
            return Err(DeserializeError::generic(
                "state marked special as a match, but has no pattern IDs",
            ));
        }
        if sp.is_match_state(id) != is_match {
            return Err(DeserializeError::generic(
                "whether state is a match or not is inconsistent",
            ));
        }

        // Now read this state's accelerator info. The first byte is the length
        // of the accelerator, which is typically 0 (for no acceleration) but
        // is no bigger than 3. The length indicates the number of bytes that
        // follow, where each byte corresponds to a transition out of this
        // state.
        if state.is_empty() {
            return Err(DeserializeError::generic("no accelerator length"));
        }
        let (accel_len, state) = (usize::from(state[0]), &state[1..]);

        if accel_len > 3 {
            return Err(DeserializeError::generic(
                "sparse invalid accelerator length",
            ));
        } else if accel_len == 0 && sp.is_accel_state(id) {
            return Err(DeserializeError::generic(
                "got no accelerators in state, but in accelerator ID range",
            ));
        } else if accel_len > 0 && !sp.is_accel_state(id) {
            return Err(DeserializeError::generic(
                "state in accelerator ID range, but has no accelerators",
            ));
        }

        wire::check_slice_len(
            state,
            accel_len,
            "sparse corrupt accelerator length",
        )?;
        let (accel, _) = (&state[..accel_len], &state[accel_len..]);

        let state = State {
            id,
            is_match,
            ntrans,
            input_ranges,
            next,
            pattern_ids,
            accel,
        };
        if sp.is_quit_state(state.next_at(state.ntrans - 1)) {
            return Err(DeserializeError::generic(
                "state with EOI transition to quit state is illegal",
            ));
        }
        Ok(state)
    }