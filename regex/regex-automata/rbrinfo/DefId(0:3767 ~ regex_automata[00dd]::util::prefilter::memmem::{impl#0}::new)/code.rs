pub(crate) fn new<B: AsRef<[u8]>>(
        _kind: MatchKind,
        needles: &[B],
    ) -> Option<Memmem> {
        #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
        {
            None
        }
        #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
        {
            if needles.len() != 1 {
                return None;
            }
            let needle = needles[0].as_ref();
            let finder = memchr::memmem::Finder::new(needle).into_owned();
            Some(Memmem { finder })
        }
    }