pub(crate) fn read_pattern_id_unchecked(slice: &[u8]) -> (PatternID, usize) {
    let pid = PatternID::from_ne_bytes_unchecked(
        slice[..PatternID::SIZE].try_into().unwrap(),
    );
    (pid, PatternID::SIZE)
}