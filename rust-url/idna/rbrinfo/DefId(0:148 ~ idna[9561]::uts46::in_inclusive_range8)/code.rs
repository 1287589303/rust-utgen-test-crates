fn in_inclusive_range8(u: u8, start: u8, end: u8) -> bool {
    u.wrapping_sub(start) <= (end - start)
}