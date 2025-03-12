fn from_bytes(bytes: [u8; 4]) -> Result<Accel, DeserializeError> {
        if usize::from(bytes[0]) >= ACCEL_LEN {
            return Err(DeserializeError::generic(
                "accelerator bytes cannot have length more than 3",
            ));
        }
        Ok(Accel::from_bytes_unchecked(bytes))
    }