pub fn write_to_native_endian(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        self.as_ref().write_to::<wire::NE>(dst)
    }