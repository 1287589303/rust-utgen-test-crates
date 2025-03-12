pub fn memory_usage(&self) -> usize {
        // The only thing that uses heap memory in a DFA is the NFA. But the
        // NFA has shared ownership, so reporting its memory as part of the
        // hybrid DFA is likely to lead to double-counting the NFA memory
        // somehow. In particular, this DFA does not really own an NFA, so
        // including it in the DFA's memory usage doesn't seem semantically
        // correct.
        0
    }