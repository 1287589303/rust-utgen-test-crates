pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        use crate::util::wire::write_state_id as write;

        if dst.len() < self.write_to_len() {
            return Err(SerializeError::buffer_too_small("special state ids"));
        }

        let mut nwrite = 0;
        nwrite += write::<E>(self.max, &mut dst[nwrite..]);
        nwrite += write::<E>(self.quit_id, &mut dst[nwrite..]);
        nwrite += write::<E>(self.min_match, &mut dst[nwrite..]);
        nwrite += write::<E>(self.max_match, &mut dst[nwrite..]);
        nwrite += write::<E>(self.min_accel, &mut dst[nwrite..]);
        nwrite += write::<E>(self.max_accel, &mut dst[nwrite..]);
        nwrite += write::<E>(self.min_start, &mut dst[nwrite..]);
        nwrite += write::<E>(self.max_start, &mut dst[nwrite..]);

        assert_eq!(
            self.write_to_len(),
            nwrite,
            "expected to write certain number of bytes",
        );
        assert_eq!(
            nwrite % 8,
            0,
            "expected to write multiple of 8 bytes for special states",
        );
        Ok(nwrite)
    }