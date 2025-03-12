pub(crate) fn new<B: AsRef<[u8]>>(
        _kind: MatchKind,
        needles: &[B],
    ) -> Option<Memchr3> {
        #[cfg(not(feature = "perf-literal-substring"))]
        {
            None
        }
        #[cfg(feature = "perf-literal-substring")]
        {
            if needles.len() != 3 {
                return None;
            }
            if !needles.iter().all(|n| n.as_ref().len() == 1) {
                return None;
            }
            let b1 = needles[0].as_ref()[0];
            let b2 = needles[1].as_ref()[0];
            let b3 = needles[2].as_ref()[0];
            Some(Memchr3(b1, b2, b3))
        }
    }