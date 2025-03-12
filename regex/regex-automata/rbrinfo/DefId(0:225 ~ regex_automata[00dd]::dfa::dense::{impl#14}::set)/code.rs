fn set(&mut self, from: StateID, unit: alphabet::Unit, to: StateID) {
        assert!(self.is_valid(from), "invalid 'from' state");
        assert!(self.is_valid(to), "invalid 'to' state");
        self.table[from.as_usize() + self.classes.get_by_unit(unit)] =
            to.as_u32();
    }