fn set_next_at(&mut self, i: usize, next: StateID) {
        let start = i * StateID::SIZE;
        let end = start + StateID::SIZE;
        wire::write_state_id::<wire::NE>(next, &mut self.next[start..end]);
    }