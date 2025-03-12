pub fn write_to_little_endian(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        self.write_to::<wire::LE>(dst)
    }