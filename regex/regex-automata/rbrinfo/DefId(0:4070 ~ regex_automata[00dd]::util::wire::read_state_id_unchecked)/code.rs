pub(crate) fn read_state_id_unchecked(slice: &[u8]) -> (StateID, usize) {
    let sid = StateID::from_ne_bytes_unchecked(
        slice[..StateID::SIZE].try_into().unwrap(),
    );
    (sid, StateID::SIZE)
}