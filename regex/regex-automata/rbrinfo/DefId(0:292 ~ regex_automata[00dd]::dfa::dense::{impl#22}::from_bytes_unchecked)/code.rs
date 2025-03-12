unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(MatchStates<&'a [u32]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();

        // Read the total number of match states.
        let (state_len, nr) =
            wire::try_read_u32_as_usize(slice, "match state length")?;
        slice = &slice[nr..];

        // Read the slice start/length pairs.
        let pair_len = wire::mul(2, state_len, "match state offset pairs")?;
        let slices_bytes_len = wire::mul(
            pair_len,
            PatternID::SIZE,
            "match state slice offset byte length",
        )?;
        wire::check_slice_len(slice, slices_bytes_len, "match state slices")?;
        wire::check_alignment::<PatternID>(slice)?;
        let slices_bytes = &slice[..slices_bytes_len];
        slice = &slice[slices_bytes_len..];
        // SAFETY: Since PatternID is always representable as a u32, all we
        // need to do is ensure that we have the proper length and alignment.
        // We've checked both above, so the cast below is safe.
        //
        // N.B. This is one of the few not-safe snippets in this function,
        // so we mark it explicitly to call it out.
        let slices = core::slice::from_raw_parts(
            slices_bytes.as_ptr().cast::<u32>(),
            pair_len,
        );

        // Read the total number of unique pattern IDs (which is always 1 more
        // than the maximum pattern ID in this automaton, since pattern IDs are
        // handed out contiguously starting at 0).
        let (pattern_len, nr) =
            wire::try_read_u32_as_usize(slice, "pattern length")?;
        slice = &slice[nr..];

        // Now read the pattern ID length. We don't need to store this
        // explicitly, but we need it to know how many pattern IDs to read.
        let (idlen, nr) =
            wire::try_read_u32_as_usize(slice, "pattern ID length")?;
        slice = &slice[nr..];

        // Read the actual pattern IDs.
        let pattern_ids_len =
            wire::mul(idlen, PatternID::SIZE, "pattern ID byte length")?;
        wire::check_slice_len(slice, pattern_ids_len, "match pattern IDs")?;
        wire::check_alignment::<PatternID>(slice)?;
        let pattern_ids_bytes = &slice[..pattern_ids_len];
        slice = &slice[pattern_ids_len..];
        // SAFETY: Since PatternID is always representable as a u32, all we
        // need to do is ensure that we have the proper length and alignment.
        // We've checked both above, so the cast below is safe.
        //
        // N.B. This is one of the few not-safe snippets in this function,
        // so we mark it explicitly to call it out.
        let pattern_ids = core::slice::from_raw_parts(
            pattern_ids_bytes.as_ptr().cast::<u32>(),
            idlen,
        );

        let ms = MatchStates { slices, pattern_ids, pattern_len };
        Ok((ms, slice.as_ptr().as_usize() - slice_start))
    }