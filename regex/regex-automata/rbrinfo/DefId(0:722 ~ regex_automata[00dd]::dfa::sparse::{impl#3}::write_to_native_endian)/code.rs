pub fn write_to_native_endian(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        self.write_to::<wire::NE>(dst)
    }