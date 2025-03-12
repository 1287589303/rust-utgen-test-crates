fn write_to<E: Endian>(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small(
                "sparse transition table",
            ));
        }
        dst = &mut dst[..nwrite];

        // write state length
        E::write_u32(u32::try_from(self.state_len).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write pattern length
        E::write_u32(u32::try_from(self.pattern_len).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write byte class map
        let n = self.classes.write_to(dst)?;
        dst = &mut dst[n..];

        // write number of bytes in sparse transitions
        E::write_u32(u32::try_from(self.sparse().len()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write actual transitions
        let mut id = DEAD;
        while id.as_usize() < self.sparse().len() {
            let state = self.state(id);
            let n = state.write_to::<E>(&mut dst)?;
            dst = &mut dst[n..];
            // The next ID is the offset immediately following `state`.
            id = StateID::new(id.as_usize() + state.write_to_len()).unwrap();
        }
        Ok(nwrite)
    }