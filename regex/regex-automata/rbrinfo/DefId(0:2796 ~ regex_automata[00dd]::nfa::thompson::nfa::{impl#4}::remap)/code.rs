fn remap(&mut self, remap: &[StateID]) {
        match *self {
            State::ByteRange { ref mut trans } => {
                trans.next = remap[trans.next]
            }
            State::Sparse(SparseTransitions { ref mut transitions }) => {
                for t in transitions.iter_mut() {
                    t.next = remap[t.next];
                }
            }
            State::Dense(DenseTransitions { ref mut transitions }) => {
                for sid in transitions.iter_mut() {
                    *sid = remap[*sid];
                }
            }
            State::Look { ref mut next, .. } => *next = remap[*next],
            State::Union { ref mut alternates } => {
                for alt in alternates.iter_mut() {
                    *alt = remap[*alt];
                }
            }
            State::BinaryUnion { ref mut alt1, ref mut alt2 } => {
                *alt1 = remap[*alt1];
                *alt2 = remap[*alt2];
            }
            State::Capture { ref mut next, .. } => *next = remap[*next],
            State::Fail => {}
            State::Match { .. } => {}
        }
    }