pub(crate) fn write_to<E: crate::util::wire::Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        use core::mem::size_of;

        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("byte set"));
        }
        let mut nw = 0;
        E::write_u128(self.bits.0[0], &mut dst[nw..]);
        nw += size_of::<u128>();
        E::write_u128(self.bits.0[1], &mut dst[nw..]);
        nw += size_of::<u128>();
        assert_eq!(nwrite, nw, "expected to write certain number of bytes",);
        assert_eq!(
            nw % 8,
            0,
            "expected to write multiple of 8 bytes for byte set",
        );
        Ok(nw)
    }