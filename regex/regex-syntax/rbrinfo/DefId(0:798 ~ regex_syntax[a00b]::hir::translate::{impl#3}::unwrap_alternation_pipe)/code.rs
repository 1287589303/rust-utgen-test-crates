fn unwrap_alternation_pipe(self) {
        match self {
            HirFrame::AlternationBranch => {}
            _ => {
                panic!(
                    "tried to unwrap alt pipe from HirFrame, got: {:?}",
                    self
                )
            }
        }
    }