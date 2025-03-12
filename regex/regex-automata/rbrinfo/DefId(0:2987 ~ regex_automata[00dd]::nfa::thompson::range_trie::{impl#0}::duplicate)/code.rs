fn duplicate(&mut self, old_id: StateID) -> StateID {
        if old_id == FINAL {
            return FINAL;
        }

        let mut stack = mem::replace(&mut self.dupe_stack, vec![]);
        stack.clear();

        let new_id = self.add_empty();
        // old_id is the state we're cloning and new_id is the ID of the
        // duplicated state for old_id.
        stack.push(NextDupe { old_id, new_id });
        while let Some(NextDupe { old_id, new_id }) = stack.pop() {
            for i in 0..self.state(old_id).transitions.len() {
                let t = self.state(old_id).transitions[i].clone();
                if t.next_id == FINAL {
                    // All final states are the same, so there's no need to
                    // duplicate it.
                    self.add_transition(new_id, t.range, FINAL);
                    continue;
                }

                let new_child_id = self.add_empty();
                self.add_transition(new_id, t.range, new_child_id);
                stack.push(NextDupe {
                    old_id: t.next_id,
                    new_id: new_child_id,
                });
            }
        }
        self.dupe_stack = stack;
        new_id
    }