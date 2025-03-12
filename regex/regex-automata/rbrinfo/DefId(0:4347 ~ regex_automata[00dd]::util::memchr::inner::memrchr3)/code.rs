pub(crate) fn memrchr3(
        n1: u8,
        n2: u8,
        n3: u8,
        haystack: &[u8],
    ) -> Option<usize> {
        memchr::memrchr3(n1, n2, n3, haystack)
    }