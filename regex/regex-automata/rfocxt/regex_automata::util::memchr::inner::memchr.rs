#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn memchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1)
}
