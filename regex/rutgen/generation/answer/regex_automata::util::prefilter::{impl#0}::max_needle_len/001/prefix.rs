// Answer 0

#[test]
fn test_max_needle_len_valid() {
    #[cfg(feature = "alloc")]
    {
        use crate::util::search::MatchKind;
        use core::sync::Arc;

        let needles = vec![b"needle1", b"needle2"];
        let prefilter = Prefilter::new(MatchKind::Any, &needles).unwrap();
        let max_len = prefilter.max_needle_len();
    }
}

#[should_panic]
#[test]
fn test_max_needle_len_invalid() {
    #[cfg(not(feature = "alloc"))]
    {
        let prefilter = Prefilter {
            _unused: (),
            pre: Arc::new(Memchr),
            is_fast: true,
            max_needle_len: 0,
        };
        let max_len = prefilter.max_needle_len();
    }
}

