#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn memrchr2(n1: u8, n2: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().rposition(|&b| b == n1 || b == n2)
}
