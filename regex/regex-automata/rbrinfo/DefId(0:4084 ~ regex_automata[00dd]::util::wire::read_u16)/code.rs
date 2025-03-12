pub(crate) fn read_u16(slice: &[u8]) -> u16 {
    let bytes: [u8; 2] = slice[..size_of::<u16>()].try_into().unwrap();
    u16::from_ne_bytes(bytes)
}