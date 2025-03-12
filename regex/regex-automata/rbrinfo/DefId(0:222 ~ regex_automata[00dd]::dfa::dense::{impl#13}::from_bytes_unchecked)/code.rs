unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(TransitionTable<&'a [u32]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();

        let (state_len, nr) =
            wire::try_read_u32_as_usize(slice, "state length")?;
        slice = &slice[nr..];

        let (stride2, nr) = wire::try_read_u32_as_usize(slice, "stride2")?;
        slice = &slice[nr..];

        let (classes, nr) = ByteClasses::from_bytes(slice)?;
        slice = &slice[nr..];

        // The alphabet length (determined by the byte class map) cannot be
        // bigger than the stride (total space used by each DFA state).
        if stride2 > 9 {
            return Err(DeserializeError::generic(
                "dense DFA has invalid stride2 (too big)",
            ));
        }
        // It also cannot be zero, since even a DFA that never matches anything
        // has a non-zero number of states with at least two equivalence
        // classes: one for all 256 byte values and another for the EOI
        // sentinel.
        if stride2 < 1 {
            return Err(DeserializeError::generic(
                "dense DFA has invalid stride2 (too small)",
            ));
        }
        // This is OK since 1 <= stride2 <= 9.
        let stride =
            1usize.checked_shl(u32::try_from(stride2).unwrap()).unwrap();
        if classes.alphabet_len() > stride {
            return Err(DeserializeError::generic(
                "alphabet size cannot be bigger than transition table stride",
            ));
        }

        let trans_len =
            wire::shl(state_len, stride2, "dense table transition length")?;
        let table_bytes_len = wire::mul(
            trans_len,
            StateID::SIZE,
            "dense table state byte length",
        )?;
        wire::check_slice_len(slice, table_bytes_len, "transition table")?;
        wire::check_alignment::<StateID>(slice)?;
        let table_bytes = &slice[..table_bytes_len];
        slice = &slice[table_bytes_len..];
        // SAFETY: Since StateID is always representable as a u32, all we need
        // to do is ensure that we have the proper length and alignment. We've
        // checked both above, so the cast below is safe.
        //
        // N.B. This is the only not-safe code in this function.
        let table = core::slice::from_raw_parts(
            table_bytes.as_ptr().cast::<u32>(),
            trans_len,
        );
        let tt = TransitionTable { table, classes, stride2 };
        Ok((tt, slice.as_ptr().as_usize() - slice_start))
    }