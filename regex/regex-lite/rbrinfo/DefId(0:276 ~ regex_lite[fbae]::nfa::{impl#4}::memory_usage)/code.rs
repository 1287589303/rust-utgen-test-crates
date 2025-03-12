fn memory_usage(&self) -> usize {
        match *self {
            State::Char { .. }
            | State::Goto { .. }
            | State::Capture { .. }
            | State::Fail { .. }
            | State::Match => 0,
            State::Splits { ref targets, .. } => {
                targets.len() * size_of::<StateID>()
            }
            State::Ranges { ref ranges, .. } => {
                ranges.len() * size_of::<(char, char)>()
            }
        }
    }