pub fn new(re: &DFA) -> Cache {
        let mut cache = Cache { explicit_slots: vec![], explicit_slot_len: 0 };
        cache.reset(re);
        cache
    }