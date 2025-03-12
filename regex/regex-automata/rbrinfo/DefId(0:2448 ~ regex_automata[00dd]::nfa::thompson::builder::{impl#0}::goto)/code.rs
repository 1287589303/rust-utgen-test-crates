fn goto(&self) -> Option<StateID> {
        match *self {
            State::Empty { next } => Some(next),
            State::Union { ref alternates } if alternates.len() == 1 => {
                Some(alternates[0])
            }
            State::UnionReverse { ref alternates }
                if alternates.len() == 1 =>
            {
                Some(alternates[0])
            }
            _ => None,
        }
    }