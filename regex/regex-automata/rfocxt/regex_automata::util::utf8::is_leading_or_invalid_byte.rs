#[cfg_attr(feature = "perf-inline", inline(always))]
fn is_leading_or_invalid_byte(b: u8) -> bool {
    (b & 0b1100_0000) != 0b1000_0000
}
