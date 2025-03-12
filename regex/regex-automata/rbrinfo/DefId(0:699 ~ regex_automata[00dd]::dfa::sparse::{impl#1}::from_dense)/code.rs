pub(crate) fn from_dense<T: AsRef<[u32]>>(
        dfa: &dense::DFA<T>,
    ) -> Result<DFA<Vec<u8>>, BuildError> {
        // In order to build the transition table, we need to be able to write
        // state identifiers for each of the "next" transitions in each state.
        // Our state identifiers correspond to the byte offset in the
        // transition table at which the state is encoded. Therefore, we do not
        // actually know what the state identifiers are until we've allocated
        // exactly as much space as we need for each state. Thus, construction
        // of the transition table happens in two passes.
        //
        // In the first pass, we fill out the shell of each state, which
        // includes the transition length, the input byte ranges and
        // zero-filled space for the transitions and accelerators, if present.
        // In this first pass, we also build up a map from the state identifier
        // index of the dense DFA to the state identifier in this sparse DFA.
        //
        // In the second pass, we fill in the transitions based on the map
        // built in the first pass.

        // The capacity given here reflects a minimum. (Well, the true minimum
        // is likely even bigger, but hopefully this saves a few reallocs.)
        let mut sparse = Vec::with_capacity(StateID::SIZE * dfa.state_len());
        // This maps state indices from the dense DFA to StateIDs in the sparse
        // DFA. We build out this map on the first pass, and then use it in the
        // second pass to back-fill our transitions.
        let mut remap: Vec<StateID> = vec![DEAD; dfa.state_len()];
        for state in dfa.states() {
            let pos = sparse.len();

            remap[dfa.to_index(state.id())] = StateID::new(pos)
                .map_err(|_| BuildError::too_many_states())?;
            // zero-filled space for the transition length
            sparse.push(0);
            sparse.push(0);

            let mut transition_len = 0;
            for (unit1, unit2, _) in state.sparse_transitions() {
                match (unit1.as_u8(), unit2.as_u8()) {
                    (Some(b1), Some(b2)) => {
                        transition_len += 1;
                        sparse.push(b1);
                        sparse.push(b2);
                    }
                    (None, None) => {}
                    (Some(_), None) | (None, Some(_)) => {
                        // can never occur because sparse_transitions never
                        // groups EOI with any other transition.
                        unreachable!()
                    }
                }
            }
            // Add dummy EOI transition. This is never actually read while
            // searching, but having space equivalent to the total number
            // of transitions is convenient. Otherwise, we'd need to track
            // a different number of transitions for the byte ranges as for
            // the 'next' states.
            //
            // N.B. The loop above is not guaranteed to yield the EOI
            // transition, since it may point to a DEAD state. By putting
            // it here, we always write the EOI transition, and thus
            // guarantee that our transition length is >0. Why do we always
            // need the EOI transition? Because in order to implement
            // Automaton::next_eoi_state, this lets us just ask for the last
            // transition. There are probably other/better ways to do this.
            transition_len += 1;
            sparse.push(0);
            sparse.push(0);

            // Check some assumptions about transition length.
            assert_ne!(
                transition_len, 0,
                "transition length should be non-zero",
            );
            assert!(
                transition_len <= 257,
                "expected transition length {} to be <= 257",
                transition_len,
            );

            // Fill in the transition length.
            // Since transition length is always <= 257, we use the most
            // significant bit to indicate whether this is a match state or
            // not.
            let ntrans = if dfa.is_match_state(state.id()) {
                transition_len | (1 << 15)
            } else {
                transition_len
            };
            wire::NE::write_u16(ntrans, &mut sparse[pos..]);

            // zero-fill the actual transitions.
            // Unwraps are OK since transition_length <= 257 and our minimum
            // support usize size is 16-bits.
            let zeros = usize::try_from(transition_len)
                .unwrap()
                .checked_mul(StateID::SIZE)
                .unwrap();
            sparse.extend(iter::repeat(0).take(zeros));

            // If this is a match state, write the pattern IDs matched by this
            // state.
            if dfa.is_match_state(state.id()) {
                let plen = dfa.match_pattern_len(state.id());
                // Write the actual pattern IDs with a u32 length prefix.
                // First, zero-fill space.
                let mut pos = sparse.len();
                // Unwraps are OK since it's guaranteed that plen <=
                // PatternID::LIMIT, which is in turn guaranteed to fit into a
                // u32.
                let zeros = size_of::<u32>()
                    .checked_mul(plen)
                    .unwrap()
                    .checked_add(size_of::<u32>())
                    .unwrap();
                sparse.extend(iter::repeat(0).take(zeros));

                // Now write the length prefix.
                wire::NE::write_u32(
                    // Will never fail since u32::MAX is invalid pattern ID.
                    // Thus, the number of pattern IDs is representable by a
                    // u32.
                    plen.try_into().expect("pattern ID length fits in u32"),
                    &mut sparse[pos..],
                );
                pos += size_of::<u32>();

                // Now write the pattern IDs.
                for &pid in dfa.pattern_id_slice(state.id()) {
                    pos += wire::write_pattern_id::<wire::NE>(
                        pid,
                        &mut sparse[pos..],
                    );
                }
            }

            // And now add the accelerator, if one exists. An accelerator is
            // at most 4 bytes and at least 1 byte. The first byte is the
            // length, N. N bytes follow the length. The set of bytes that
            // follow correspond (exhaustively) to the bytes that must be seen
            // to leave this state.
            let accel = dfa.accelerator(state.id());
            sparse.push(accel.len().try_into().unwrap());
            sparse.extend_from_slice(accel);
        }

        let mut new = DFA {
            tt: Transitions {
                sparse,
                classes: dfa.byte_classes().clone(),
                state_len: dfa.state_len(),
                pattern_len: dfa.pattern_len(),
            },
            st: StartTable::from_dense_dfa(dfa, &remap)?,
            special: dfa.special().remap(|id| remap[dfa.to_index(id)]),
            pre: dfa.get_prefilter().map(|p| p.clone()),
            quitset: dfa.quitset().clone(),
            flags: dfa.flags().clone(),
        };
        // And here's our second pass. Iterate over all of the dense states
        // again, and update the transitions in each of the states in the
        // sparse DFA.
        for old_state in dfa.states() {
            let new_id = remap[dfa.to_index(old_state.id())];
            let mut new_state = new.tt.state_mut(new_id);
            let sparse = old_state.sparse_transitions();
            for (i, (_, _, next)) in sparse.enumerate() {
                let next = remap[dfa.to_index(next)];
                new_state.set_next_at(i, next);
            }
        }
        debug!(
            "created sparse DFA, memory usage: {} (dense memory usage: {})",
            new.memory_usage(),
            dfa.memory_usage(),
        );
        Ok(new)
    }