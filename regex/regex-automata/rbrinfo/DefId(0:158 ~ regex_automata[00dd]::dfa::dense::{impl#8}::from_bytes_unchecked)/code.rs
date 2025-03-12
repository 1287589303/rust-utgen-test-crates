pub unsafe fn from_bytes_unchecked(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u32]>, usize), DeserializeError> {
        let mut nr = 0;

        nr += wire::skip_initial_padding(slice);
        wire::check_alignment::<StateID>(&slice[nr..])?;
        nr += wire::read_label(&slice[nr..], LABEL)?;
        nr += wire::read_endianness_check(&slice[nr..])?;
        nr += wire::read_version(&slice[nr..], VERSION)?;

        let _unused = wire::try_read_u32(&slice[nr..], "unused space")?;
        nr += size_of::<u32>();

        let (flags, nread) = Flags::from_bytes(&slice[nr..])?;
        nr += nread;

        let (tt, nread) = TransitionTable::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (st, nread) = StartTable::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (ms, nread) = MatchStates::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (special, nread) = Special::from_bytes(&slice[nr..])?;
        nr += nread;
        special.validate_state_len(tt.len(), tt.stride2)?;

        let (accels, nread) = Accels::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (quitset, nread) = ByteSet::from_bytes(&slice[nr..])?;
        nr += nread;

        // Prefilters don't support serialization, so they're always absent.
        let pre = None;
        Ok((DFA { tt, st, ms, special, accels, pre, quitset, flags }, nr))
    }