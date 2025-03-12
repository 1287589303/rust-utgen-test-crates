pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseDFAEngine> {
        let engine = self.0.as_ref()?;
        Some(engine)
    }