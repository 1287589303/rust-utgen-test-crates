pub(crate) fn reset(&mut self, builder: &BoundedBacktracker) {
        #[cfg(feature = "nfa-backtrack")]
        if let Some(ref e) = builder.0 {
            self.0.as_mut().unwrap().reset(&e.0);
        }
    }