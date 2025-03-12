unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(StartTable<&'a [u8]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();

        let (kind, nr) = StartKind::from_bytes(slice)?;
        slice = &slice[nr..];

        let (start_map, nr) = StartByteMap::from_bytes(slice)?;
        slice = &slice[nr..];

        let (stride, nr) =
            wire::try_read_u32_as_usize(slice, "sparse start table stride")?;
        slice = &slice[nr..];
        if stride != Start::len() {
            return Err(DeserializeError::generic(
                "invalid sparse starting table stride",
            ));
        }

        let (maybe_pattern_len, nr) =
            wire::try_read_u32_as_usize(slice, "sparse start table patterns")?;
        slice = &slice[nr..];
        let pattern_len = if maybe_pattern_len.as_u32() == u32::MAX {
            None
        } else {
            Some(maybe_pattern_len)
        };
        if pattern_len.map_or(false, |len| len > PatternID::LIMIT) {
            return Err(DeserializeError::generic(
                "sparse invalid number of patterns",
            ));
        }

        let (universal_unanchored, nr) =
            wire::try_read_u32(slice, "universal unanchored start")?;
        slice = &slice[nr..];
        let universal_start_unanchored = if universal_unanchored == u32::MAX {
            None
        } else {
            Some(StateID::try_from(universal_unanchored).map_err(|e| {
                DeserializeError::state_id_error(
                    e,
                    "universal unanchored start",
                )
            })?)
        };

        let (universal_anchored, nr) =
            wire::try_read_u32(slice, "universal anchored start")?;
        slice = &slice[nr..];
        let universal_start_anchored = if universal_anchored == u32::MAX {
            None
        } else {
            Some(StateID::try_from(universal_anchored).map_err(|e| {
                DeserializeError::state_id_error(e, "universal anchored start")
            })?)
        };

        let pattern_table_size = wire::mul(
            stride,
            pattern_len.unwrap_or(0),
            "sparse invalid pattern length",
        )?;
        // Our start states always start with a single stride of start states
        // for the entire automaton which permit it to match any pattern. What
        // follows it are an optional set of start states for each pattern.
        let start_state_len = wire::add(
            wire::mul(2, stride, "start state stride too big")?,
            pattern_table_size,
            "sparse invalid 'any' pattern starts size",
        )?;
        let table_bytes_len = wire::mul(
            start_state_len,
            StateID::SIZE,
            "sparse pattern table bytes length",
        )?;
        wire::check_slice_len(
            slice,
            table_bytes_len,
            "sparse start ID table",
        )?;
        let table = &slice[..table_bytes_len];
        slice = &slice[table_bytes_len..];

        let sl = StartTable {
            table,
            kind,
            start_map,
            stride,
            pattern_len,
            universal_start_unanchored,
            universal_start_anchored,
        };
        Ok((sl, slice.as_ptr().as_usize() - slice_start))
    }