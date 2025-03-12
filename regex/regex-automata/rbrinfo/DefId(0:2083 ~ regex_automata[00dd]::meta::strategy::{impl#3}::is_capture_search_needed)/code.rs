fn is_capture_search_needed(&self, slots_len: usize) -> bool {
        slots_len > self.nfa.group_info().implicit_slot_len()
    }