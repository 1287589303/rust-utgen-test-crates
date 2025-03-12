pub fn reset(&mut self, re: &DFA) {
        let explicit_slot_len = re.get_nfa().group_info().explicit_slot_len();
        self.explicit_slots.resize(explicit_slot_len, None);
        self.explicit_slot_len = explicit_slot_len;
    }