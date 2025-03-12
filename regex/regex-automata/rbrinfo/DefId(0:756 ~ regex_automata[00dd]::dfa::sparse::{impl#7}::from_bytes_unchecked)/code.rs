unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(Transitions<&'a [u8]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();

        let (state_len, nr) =
            wire::try_read_u32_as_usize(&slice, "state length")?;
        slice = &slice[nr..];

        let (pattern_len, nr) =
            wire::try_read_u32_as_usize(&slice, "pattern length")?;
        slice = &slice[nr..];

        let (classes, nr) = ByteClasses::from_bytes(&slice)?;
        slice = &slice[nr..];

        let (len, nr) =
            wire::try_read_u32_as_usize(&slice, "sparse transitions length")?;
        slice = &slice[nr..];

        wire::check_slice_len(slice, len, "sparse states byte length")?;
        let sparse = &slice[..len];
        slice = &slice[len..];

        let trans = Transitions { sparse, classes, state_len, pattern_len };
        Ok((trans, slice.as_ptr().as_usize() - slice_start))
    }