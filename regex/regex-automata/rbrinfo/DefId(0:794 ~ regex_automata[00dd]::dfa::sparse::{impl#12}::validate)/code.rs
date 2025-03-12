fn validate(
        &self,
        sp: &Special,
        seen: &Seen,
    ) -> Result<(), DeserializeError> {
        for (id, _, _) in self.iter() {
            if !seen.contains(&id) {
                return Err(DeserializeError::generic(
                    "found invalid start state ID",
                ));
            }
            if sp.is_match_state(id) {
                return Err(DeserializeError::generic(
                    "start states cannot be match states",
                ));
            }
        }
        Ok(())
    }