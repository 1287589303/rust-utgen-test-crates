fn add_empty_state(&mut self) -> Result<StateID, BuildError> {
        // Normally, to get a fresh state identifier, we would just
        // take the index of the next state added to the transition
        // table. However, we actually perform an optimization here
        // that premultiplies state IDs by the stride, such that they
        // point immediately at the beginning of their transitions in
        // the transition table. This avoids an extra multiplication
        // instruction for state lookup at search time.
        //
        // Premultiplied identifiers means that instead of your matching
        // loop looking something like this:
        //
        //   state = dfa.start
        //   for byte in haystack:
        //       next = dfa.transitions[state * stride + byte]
        //       if dfa.is_match(next):
        //           return true
        //   return false
        //
        // it can instead look like this:
        //
        //   state = dfa.start
        //   for byte in haystack:
        //       next = dfa.transitions[state + byte]
        //       if dfa.is_match(next):
        //           return true
        //   return false
        //
        // In other words, we save a multiplication instruction in the
        // critical path. This turns out to be a decent performance win.
        // The cost of using premultiplied state ids is that they can
        // require a bigger state id representation. (And they also make
        // the code a bit more complex, especially during minimization and
        // when reshuffling states, as one needs to convert back and forth
        // between state IDs and state indices.)
        //
        // To do this, we simply take the index of the state into the
        // entire transition table, rather than the index of the state
        // itself. e.g., If the stride is 64, then the ID of the 3rd state
        // is 192, not 2.
        let next = self.table.len();
        let id =
            StateID::new(next).map_err(|_| BuildError::too_many_states())?;
        self.table.extend(iter::repeat(0).take(self.stride()));
        Ok(id)
    }