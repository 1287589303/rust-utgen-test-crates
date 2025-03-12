pub(crate) fn none() -> OnePassCache {
        #[cfg(feature = "dfa-onepass")]
        {
            OnePassCache(None)
        }
        #[cfg(not(feature = "dfa-onepass"))]
        {
            OnePassCache(())
        }
    }