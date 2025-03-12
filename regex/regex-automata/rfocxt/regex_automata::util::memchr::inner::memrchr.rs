#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn memrchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().rposition(|&b| b == n1)
}
