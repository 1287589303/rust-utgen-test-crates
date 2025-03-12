fn memory_usage(&self) -> usize {
        match *self {
            State::Empty { .. }
            | State::ByteRange { .. }
            | State::Look { .. }
            | State::CaptureStart { .. }
            | State::CaptureEnd { .. }
            | State::Fail
            | State::Match { .. } => 0,
            State::Sparse { ref transitions } => {
                transitions.len() * mem::size_of::<Transition>()
            }
            State::Union { ref alternates } => {
                alternates.len() * mem::size_of::<StateID>()
            }
            State::UnionReverse { ref alternates } => {
                alternates.len() * mem::size_of::<StateID>()
            }
        }
    }