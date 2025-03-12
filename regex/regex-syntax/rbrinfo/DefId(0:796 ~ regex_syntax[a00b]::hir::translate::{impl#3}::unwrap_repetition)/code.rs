fn unwrap_repetition(self) {
        match self {
            HirFrame::Repetition => {}
            _ => {
                panic!(
                    "tried to unwrap repetition from HirFrame, got: {:?}",
                    self
                )
            }
        }
    }