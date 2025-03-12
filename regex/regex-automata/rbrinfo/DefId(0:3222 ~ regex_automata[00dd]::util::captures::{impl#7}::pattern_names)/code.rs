pub fn pattern_names(&self, pid: PatternID) -> GroupInfoPatternNames<'_> {
        GroupInfoPatternNames {
            it: self
                .0
                .index_to_name
                .get(pid.as_usize())
                .map(|indices| indices.iter())
                .unwrap_or([].iter()),
        }
    }