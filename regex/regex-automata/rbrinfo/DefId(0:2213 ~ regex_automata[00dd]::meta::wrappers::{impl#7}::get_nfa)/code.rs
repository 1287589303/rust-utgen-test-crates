fn get_nfa(&self) -> &NFA {
        #[cfg(feature = "dfa-onepass")]
        {
            self.0.get_nfa()
        }
        #[cfg(not(feature = "dfa-onepass"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }