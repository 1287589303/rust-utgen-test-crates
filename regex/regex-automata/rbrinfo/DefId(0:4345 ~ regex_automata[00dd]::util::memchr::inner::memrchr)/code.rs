pub(crate) fn memrchr(n1: u8, haystack: &[u8]) -> Option<usize> {
        memchr::memrchr(n1, haystack)
    }