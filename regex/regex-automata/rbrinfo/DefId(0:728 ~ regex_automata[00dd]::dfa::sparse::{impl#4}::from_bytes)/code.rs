pub fn from_bytes(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u8]>, usize), DeserializeError> {
        // SAFETY: This is safe because we validate both the sparse transitions
        // (by trying to decode every state) and start state ID list below. If
        // either validation fails, then we return an error.
        let (dfa, nread) = unsafe { DFA::from_bytes_unchecked(slice)? };
        let seen = dfa.tt.validate(&dfa.special)?;
        dfa.st.validate(&dfa.special, &seen)?;
        // N.B. dfa.special doesn't have a way to do unchecked deserialization,
        // so it has already been validated.
        Ok((dfa, nread))
    }