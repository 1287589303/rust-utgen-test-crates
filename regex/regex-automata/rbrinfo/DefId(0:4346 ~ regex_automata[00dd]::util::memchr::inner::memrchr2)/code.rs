pub(crate) fn memrchr2(n1: u8, n2: u8, haystack: &[u8]) -> Option<usize> {
        memchr::memrchr2(n1, n2, haystack)
    }