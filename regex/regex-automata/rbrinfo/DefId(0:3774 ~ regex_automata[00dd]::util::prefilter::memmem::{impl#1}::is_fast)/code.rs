fn is_fast(&self) -> bool {
        #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
        {
            unreachable!()
        }
        #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
        {
            true
        }
    }