pub fn rank(byte: u8) -> u8 {
    crate::rank::BYTE_FREQUENCIES[usize::from(byte)]
}