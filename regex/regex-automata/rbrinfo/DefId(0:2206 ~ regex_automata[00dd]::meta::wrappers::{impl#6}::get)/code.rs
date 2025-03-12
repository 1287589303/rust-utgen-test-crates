pub(crate) fn get(&self, input: &Input<'_>) -> Option<&OnePassEngine> {
        let engine = self.0.as_ref()?;
        if !input.get_anchored().is_anchored()
            && !engine.get_nfa().is_always_start_anchored()
        {
            return None;
        }
        Some(engine)
    }