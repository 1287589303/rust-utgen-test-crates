fn is_fast(&self) -> bool {
        #[cfg(not(feature = "perf-literal-multisubstring"))]
        {
            unreachable!()
        }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            // Aho-Corasick is never considered "fast" because it's never
            // going to be even close to an order of magnitude faster than the
            // regex engine itself (assuming a DFA is used). In fact, it is
            // usually slower. The magic of Aho-Corasick is that it can search
            // a *large* number of literals with a relatively small amount of
            // memory. The regex engines are far more wasteful.
            //
            // Aho-Corasick may be "fast" when the regex engine corresponds
            // to, say, the PikeVM. That happens when the lazy DFA couldn't be
            // built or used for some reason. But in these cases, the regex
            // itself is likely quite big and we're probably hosed no matter
            // what we do. (In this case, the best bet is for the caller to
            // increase some of the memory limits on the hybrid cache capacity
            // and hope that's enough.)
            false
        }
    }