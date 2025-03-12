fn memory_usage(&self) -> usize {
        match *self {
            State::ByteRange { .. }
            | State::Look { .. }
            | State::BinaryUnion { .. }
            | State::Capture { .. }
            | State::Match { .. }
            | State::Fail => 0,
            State::Sparse(SparseTransitions { ref transitions }) => {
                transitions.len() * mem::size_of::<Transition>()
            }
            State::Dense { .. } => 256 * mem::size_of::<StateID>(),
            State::Union { ref alternates } => {
                alternates.len() * mem::size_of::<StateID>()
            }
        }
    }