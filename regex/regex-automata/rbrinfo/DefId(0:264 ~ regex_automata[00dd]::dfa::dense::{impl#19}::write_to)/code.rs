fn write_to<E: Endian>(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small(
                "starting table ids",
            ));
        }
        dst = &mut dst[..nwrite];

        // write start kind
        let nw = self.kind.write_to::<E>(dst)?;
        dst = &mut dst[nw..];
        // write start byte map
        let nw = self.start_map.write_to(dst)?;
        dst = &mut dst[nw..];
        // write stride
        // Unwrap is OK since the stride is always 4 (currently).
        E::write_u32(u32::try_from(self.stride).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];
        // write pattern length
        // Unwrap is OK since number of patterns is guaranteed to fit in a u32.
        E::write_u32(
            u32::try_from(self.pattern_len.unwrap_or(0xFFFF_FFFF)).unwrap(),
            dst,
        );
        dst = &mut dst[size_of::<u32>()..];
        // write universal start unanchored state id, u32::MAX if absent
        E::write_u32(
            self.universal_start_unanchored
                .map_or(u32::MAX, |sid| sid.as_u32()),
            dst,
        );
        dst = &mut dst[size_of::<u32>()..];
        // write universal start anchored state id, u32::MAX if absent
        E::write_u32(
            self.universal_start_anchored.map_or(u32::MAX, |sid| sid.as_u32()),
            dst,
        );
        dst = &mut dst[size_of::<u32>()..];
        // write start IDs
        for &sid in self.table() {
            let n = wire::write_state_id::<E>(sid, &mut dst);
            dst = &mut dst[n..];
        }
        Ok(nwrite)
    }