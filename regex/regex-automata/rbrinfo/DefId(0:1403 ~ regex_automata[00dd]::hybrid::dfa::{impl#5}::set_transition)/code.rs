fn set_transition(
        &mut self,
        from: LazyStateID,
        unit: alphabet::Unit,
        to: LazyStateID,
    ) {
        assert!(self.as_ref().is_valid(from), "invalid 'from' id: {:?}", from);
        assert!(self.as_ref().is_valid(to), "invalid 'to' id: {:?}", to);
        let offset =
            from.as_usize_untagged() + self.dfa.classes.get_by_unit(unit);
        self.cache.trans[offset] = to;
    }