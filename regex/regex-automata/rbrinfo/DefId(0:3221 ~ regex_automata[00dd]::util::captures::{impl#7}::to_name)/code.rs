pub fn to_name(&self, pid: PatternID, group_index: usize) -> Option<&str> {
        let pattern_names = self.0.index_to_name.get(pid.as_usize())?;
        pattern_names.get(group_index)?.as_deref()
    }