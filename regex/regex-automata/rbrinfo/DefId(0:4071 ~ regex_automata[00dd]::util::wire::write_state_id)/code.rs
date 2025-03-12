pub(crate) fn write_state_id<E: Endian>(
    sid: StateID,
    dst: &mut [u8],
) -> usize {
    E::write_u32(sid.as_u32(), dst);
    StateID::SIZE
}