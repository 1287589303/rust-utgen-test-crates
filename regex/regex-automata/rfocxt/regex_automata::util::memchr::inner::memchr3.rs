#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn memchr3(n1: u8, n2: u8, n3: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1 || b == n2 || b == n3)
}
