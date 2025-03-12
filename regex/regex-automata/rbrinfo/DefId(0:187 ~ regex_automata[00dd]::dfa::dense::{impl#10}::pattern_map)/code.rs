pub(crate) fn pattern_map(&self) -> BTreeMap<StateID, Vec<PatternID>> {
        self.ms.to_map(self)
    }