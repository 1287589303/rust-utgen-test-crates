fn apply(
        self,
        at: usize,
        caller_explicit_slots: &mut [Option<NonMaxUsize>],
    ) {
        if self.is_empty() {
            return;
        }
        let at = NonMaxUsize::new(at);
        for slot in self.iter() {
            if slot >= caller_explicit_slots.len() {
                break;
            }
            caller_explicit_slots[slot] = at;
        }
    }