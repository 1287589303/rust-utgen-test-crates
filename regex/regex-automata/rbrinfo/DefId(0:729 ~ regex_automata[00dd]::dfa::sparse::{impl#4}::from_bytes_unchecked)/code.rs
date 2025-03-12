pub unsafe fn from_bytes_unchecked(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u8]>, usize), DeserializeError> {
        let mut nr = 0;

        nr += wire::read_label(&slice[nr..], LABEL)?;
        nr += wire::read_endianness_check(&slice[nr..])?;
        nr += wire::read_version(&slice[nr..], VERSION)?;

        let _unused = wire::try_read_u32(&slice[nr..], "unused space")?;
        nr += size_of::<u32>();

        let (flags, nread) = Flags::from_bytes(&slice[nr..])?;
        nr += nread;

        let (tt, nread) = Transitions::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (st, nread) = StartTable::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (special, nread) = Special::from_bytes(&slice[nr..])?;
        nr += nread;
        if special.max.as_usize() >= tt.sparse().len() {
            return Err(DeserializeError::generic(
                "max should not be greater than or equal to sparse bytes",
            ));
        }

        let (quitset, nread) = ByteSet::from_bytes(&slice[nr..])?;
        nr += nread;

        // Prefilters don't support serialization, so they're always absent.
        let pre = None;
        Ok((DFA { tt, st, special, pre, quitset, flags }, nr))
    }