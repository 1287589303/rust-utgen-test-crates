pub(crate) fn matches_unit(
        &self,
        unit: alphabet::Unit,
    ) -> Option<StateID> {
        unit.as_u8().map_or(None, |byte| self.matches_byte(byte))
    }