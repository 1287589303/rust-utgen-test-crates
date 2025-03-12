fn memory_usage(&self) -> usize {
        #[cfg(not(feature = "perf-literal-multisubstring"))]
        {
            unreachable!()
        }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            use aho_corasick::automaton::Automaton;
            self.searcher.memory_usage() + self.anchored_ac.memory_usage()
        }
    }