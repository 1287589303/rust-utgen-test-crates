pub(crate) fn write_pattern_id<E: Endian>(
    pid: PatternID,
    dst: &mut [u8],
) -> usize {
    E::write_u32(pid.as_u32(), dst);
    PatternID::SIZE
}