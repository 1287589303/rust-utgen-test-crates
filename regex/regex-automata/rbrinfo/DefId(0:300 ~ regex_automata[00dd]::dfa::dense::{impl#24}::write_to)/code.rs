fn write_to<E: Endian>(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("match states"));
        }
        dst = &mut dst[..nwrite];

        // write state ID length
        // Unwrap is OK since number of states is guaranteed to fit in a u32.
        E::write_u32(u32::try_from(self.len()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write slice offset pairs
        for &pid in self.slices() {
            let n = wire::write_pattern_id::<E>(pid, &mut dst);
            dst = &mut dst[n..];
        }

        // write unique pattern ID length
        // Unwrap is OK since number of patterns is guaranteed to fit in a u32.
        E::write_u32(u32::try_from(self.pattern_len).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write pattern ID length
        // Unwrap is OK since we check at construction (and deserialization)
        // that the number of patterns is representable as a u32.
        E::write_u32(u32::try_from(self.pattern_ids().len()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write pattern IDs
        for &pid in self.pattern_ids() {
            let n = wire::write_pattern_id::<E>(pid, &mut dst);
            dst = &mut dst[n..];
        }

        Ok(nwrite)
    }