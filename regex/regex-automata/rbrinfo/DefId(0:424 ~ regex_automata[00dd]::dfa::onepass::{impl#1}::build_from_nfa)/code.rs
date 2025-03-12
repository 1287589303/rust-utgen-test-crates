pub fn build_from_nfa(&self, nfa: NFA) -> Result<DFA, BuildError> {
        // Why take ownership if we're just going to pass a reference to the
        // NFA to our internal builder? Well, the first thing to note is that
        // an NFA uses reference counting internally, so either choice is going
        // to be cheap. So there isn't much cost either way.
        //
        // The real reason is that a one-pass DFA, semantically, shares
        // ownership of an NFA. This is unlike other DFAs that don't share
        // ownership of an NFA at all, primarily because they want to be
        // self-contained in order to support cheap (de)serialization.
        //
        // But then why pass a '&nfa' below if we want to share ownership?
        // Well, it turns out that using a '&NFA' in our internal builder
        // separates its lifetime from the DFA we're building, and this turns
        // out to make code a bit more composable. e.g., We can iterate over
        // things inside the NFA while borrowing the builder as mutable because
        // we know the NFA cannot be mutated. So TL;DR --- this weirdness is
        // "because borrow checker."
        InternalBuilder::new(self.config.clone(), &nfa).build()
    }