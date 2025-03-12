fn table_mut(&mut self) -> &mut [StateID] {
        wire::u32s_to_state_ids_mut(self.table.as_mut())
    }