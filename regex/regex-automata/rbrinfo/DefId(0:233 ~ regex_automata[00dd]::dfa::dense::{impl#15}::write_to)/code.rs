fn write_to<E: Endian>(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("transition table"));
        }
        dst = &mut dst[..nwrite];

        // write state length
        // Unwrap is OK since number of states is guaranteed to fit in a u32.
        E::write_u32(u32::try_from(self.len()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write state stride (as power of 2)
        // Unwrap is OK since stride2 is guaranteed to be <= 9.
        E::write_u32(u32::try_from(self.stride2).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write byte class map
        let n = self.classes.write_to(dst)?;
        dst = &mut dst[n..];

        // write actual transitions
        for &sid in self.table() {
            let n = wire::write_state_id::<E>(sid, &mut dst);
            dst = &mut dst[n..];
        }
        Ok(nwrite)
    }