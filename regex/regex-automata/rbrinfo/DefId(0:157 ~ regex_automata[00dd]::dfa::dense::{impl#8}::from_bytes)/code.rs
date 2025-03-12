pub fn from_bytes(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u32]>, usize), DeserializeError> {
        // SAFETY: This is safe because we validate the transition table, start
        // table, match states and accelerators below. If any validation fails,
        // then we return an error.
        let (dfa, nread) = unsafe { DFA::from_bytes_unchecked(slice)? };
        dfa.tt.validate(&dfa)?;
        dfa.st.validate(&dfa)?;
        dfa.ms.validate(&dfa)?;
        dfa.accels.validate()?;
        // N.B. dfa.special doesn't have a way to do unchecked deserialization,
        // so it has already been validated.
        for state in dfa.states() {
            // If the state is an accel state, then it must have a non-empty
            // accelerator.
            if dfa.is_accel_state(state.id()) {
                let index = dfa.accelerator_index(state.id());
                if index >= dfa.accels.len() {
                    return Err(DeserializeError::generic(
                        "found DFA state with invalid accelerator index",
                    ));
                }
                let needles = dfa.accels.needles(index);
                if !(1 <= needles.len() && needles.len() <= 3) {
                    return Err(DeserializeError::generic(
                        "accelerator needles has invalid length",
                    ));
                }
            }
        }
        Ok((dfa, nread))
    }