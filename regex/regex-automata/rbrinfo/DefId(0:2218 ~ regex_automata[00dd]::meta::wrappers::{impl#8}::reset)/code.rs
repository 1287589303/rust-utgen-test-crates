pub(crate) fn reset(&mut self, builder: &OnePass) {
        #[cfg(feature = "dfa-onepass")]
        if let Some(ref e) = builder.0 {
            self.0.as_mut().unwrap().reset(&e.0);
        }
    }