pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("start kind"));
        }
        let n = match *self {
            StartKind::Both => 0,
            StartKind::Unanchored => 1,
            StartKind::Anchored => 2,
        };
        E::write_u32(n, dst);
        Ok(nwrite)
    }