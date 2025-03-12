#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn is_boundary(bytes: &[u8], i: usize) -> bool {
    match bytes.get(i) {
        None => i == bytes.len(),
        Some(&b) => b <= 0b0111_1111 || b >= 0b1100_0000,
    }
}
