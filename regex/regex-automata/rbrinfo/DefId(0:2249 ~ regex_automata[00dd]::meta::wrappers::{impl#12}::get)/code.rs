pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&DFAEngine> {
        let engine = self.0.as_ref()?;
        Some(engine)
    }