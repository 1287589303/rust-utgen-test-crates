fn patch(&self, from: StateID, to: StateID) -> Result<(), Error> {
        let mut new_memory_extra = self.nfa.borrow().memory_extra;
        match self.nfa.borrow_mut().states[from.as_usize()] {
            State::Char { ref mut target, .. } => {
                *target = to;
            }
            State::Ranges { ref mut target, .. } => {
                *target = to;
            }
            State::Splits { ref mut targets, .. } => {
                targets.push(to);
                new_memory_extra += size_of::<StateID>();
            }
            State::Goto { ref mut target, .. } => {
                *target = to;
            }
            State::Capture { ref mut target, .. } => {
                *target = to;
            }
            State::Fail | State::Match => {}
        }
        if new_memory_extra != self.nfa.borrow().memory_extra {
            self.nfa.borrow_mut().memory_extra = new_memory_extra;
            self.check_size_limit()?;
        }
        Ok(())
    }