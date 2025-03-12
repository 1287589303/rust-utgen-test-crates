pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteSet, usize), DeserializeError> {
        use core::mem::size_of;

        wire::check_slice_len(slice, 2 * size_of::<u128>(), "byte set")?;
        let mut nread = 0;
        let (low, nr) = wire::try_read_u128(slice, "byte set low bucket")?;
        nread += nr;
        let (high, nr) = wire::try_read_u128(slice, "byte set high bucket")?;
        nread += nr;
        Ok((ByteSet { bits: BitSet([low, high]) }, nread))
    }