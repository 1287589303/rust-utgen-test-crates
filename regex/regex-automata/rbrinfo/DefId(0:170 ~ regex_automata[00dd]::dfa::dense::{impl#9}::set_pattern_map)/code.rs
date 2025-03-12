pub(crate) fn set_pattern_map(
        &mut self,
        map: &BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<(), BuildError> {
        self.ms = self.ms.new_with_map(map)?;
        Ok(())
    }