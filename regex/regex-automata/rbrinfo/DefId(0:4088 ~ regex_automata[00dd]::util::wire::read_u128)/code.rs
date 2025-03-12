pub(crate) fn read_u128(slice: &[u8]) -> u128 {
    let bytes: [u8; 16] = slice[..size_of::<u128>()].try_into().unwrap();
    u128::from_ne_bytes(bytes)
}