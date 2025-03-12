pub fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        assert_eq!(
            nwrite % ACCEL_TY_SIZE,
            0,
            "expected accelerator bytes written to be a multiple of {}",
            ACCEL_TY_SIZE,
        );
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("accelerators"));
        }

        // The number of accelerators can never exceed AccelTy::MAX.
        E::write_u32(AccelTy::try_from(self.len()).unwrap(), dst);
        // The actual accelerators are just raw bytes and thus their endianness
        // is irrelevant. So we can copy them as bytes.
        dst[ACCEL_TY_SIZE..nwrite]
            .copy_from_slice(&self.as_bytes()[ACCEL_TY_SIZE..nwrite]);
        Ok(nwrite)
    }