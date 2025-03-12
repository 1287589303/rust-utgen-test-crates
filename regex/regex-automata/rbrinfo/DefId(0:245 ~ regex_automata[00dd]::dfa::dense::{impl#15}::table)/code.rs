fn table(&self) -> &[StateID] {
        wire::u32s_to_state_ids(self.table.as_ref())
    }