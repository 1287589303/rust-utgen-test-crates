pub(super) fn add(&mut self, state: State) -> StateID {
        match state {
            State::ByteRange { ref trans } => {
                self.byte_class_set.set_range(trans.start, trans.end);
            }
            State::Sparse(ref sparse) => {
                for trans in sparse.transitions.iter() {
                    self.byte_class_set.set_range(trans.start, trans.end);
                }
            }
            State::Dense { .. } => unreachable!(),
            State::Look { look, .. } => {
                self.look_matcher
                    .add_to_byteset(look, &mut self.byte_class_set);
                self.look_set_any = self.look_set_any.insert(look);
            }
            State::Capture { .. } => {
                self.has_capture = true;
            }
            State::Union { .. }
            | State::BinaryUnion { .. }
            | State::Fail
            | State::Match { .. } => {}
        }

        let id = StateID::new(self.states.len()).unwrap();
        self.memory_extra += state.memory_usage();
        self.states.push(state);
        id
    }