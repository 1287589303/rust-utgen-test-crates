pub(crate) fn try_read_state_id(
    slice: &[u8],
    what: &'static str,
) -> Result<(StateID, usize), DeserializeError> {
    if slice.len() < StateID::SIZE {
        return Err(DeserializeError::buffer_too_small(what));
    }
    read_state_id(slice, what)
}