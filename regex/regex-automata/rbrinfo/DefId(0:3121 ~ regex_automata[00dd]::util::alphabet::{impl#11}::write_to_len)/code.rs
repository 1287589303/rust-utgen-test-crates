pub(crate) fn write_to_len(&self) -> usize {
        2 * core::mem::size_of::<u128>()
    }