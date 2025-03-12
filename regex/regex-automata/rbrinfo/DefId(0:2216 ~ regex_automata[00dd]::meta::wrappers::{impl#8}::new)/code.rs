pub(crate) fn new(builder: &OnePass) -> OnePassCache {
        #[cfg(feature = "dfa-onepass")]
        {
            OnePassCache(builder.0.as_ref().map(|e| e.0.create_cache()))
        }
        #[cfg(not(feature = "dfa-onepass"))]
        {
            OnePassCache(())
        }
    }