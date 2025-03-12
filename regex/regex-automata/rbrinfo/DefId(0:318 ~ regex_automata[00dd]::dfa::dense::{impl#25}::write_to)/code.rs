pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        fn bool_to_int(b: bool) -> u32 {
            if b {
                1
            } else {
                0
            }
        }

        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("flag bitset"));
        }
        let bits = (bool_to_int(self.has_empty) << 0)
            | (bool_to_int(self.is_utf8) << 1)
            | (bool_to_int(self.is_always_start_anchored) << 2);
        E::write_u32(bits, dst);
        Ok(nwrite)
    }