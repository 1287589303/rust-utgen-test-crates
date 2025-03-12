pub fn iter<E, F: FnMut(&[Utf8Range]) -> Result<(), E>>(
        &self,
        mut f: F,
    ) -> Result<(), E> {
        let mut stack = self.iter_stack.borrow_mut();
        stack.clear();
        let mut ranges = self.iter_ranges.borrow_mut();
        ranges.clear();

        // We do iteration in a way that permits us to use a single buffer
        // for our keys. We iterate in a depth first fashion, while being
        // careful to expand our frontier as we move deeper in the trie.
        stack.push(NextIter { state_id: ROOT, tidx: 0 });
        while let Some(NextIter { mut state_id, mut tidx }) = stack.pop() {
            // This could be implemented more simply without an inner loop
            // here, but at the cost of more stack pushes.
            loop {
                let state = self.state(state_id);
                // If we've visited all transitions in this state, then pop
                // back to the parent state.
                if tidx >= state.transitions.len() {
                    ranges.pop();
                    break;
                }

                let t = &state.transitions[tidx];
                ranges.push(t.range);
                if t.next_id == FINAL {
                    f(&ranges)?;
                    ranges.pop();
                    tidx += 1;
                } else {
                    // Expand our frontier. Once we come back to this state
                    // via the stack, start in on the next transition.
                    stack.push(NextIter { state_id, tidx: tidx + 1 });
                    // Otherwise, move to the first transition of the next
                    // state.
                    state_id = t.next_id;
                    tidx = 0;
                }
            }
        }
        Ok(())
    }