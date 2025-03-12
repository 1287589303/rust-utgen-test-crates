pub fn to_index(&self, pid: PatternID, name: &str) -> Option<usize> {
        let indices = self.0.name_to_index.get(pid.as_usize())?;
        indices.get(name).cloned().map(|i| i.as_usize())
    }