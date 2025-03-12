pub fn write_to_big_endian(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        self.as_ref().write_to::<wire::BE>(dst)
    }