fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let x = match *self {
            ClassFrame::Union { .. } => "Union",
            ClassFrame::Binary { .. } => "Binary",
            ClassFrame::BinaryLHS { .. } => "BinaryLHS",
            ClassFrame::BinaryRHS { .. } => "BinaryRHS",
        };
        write!(f, "{}", x)
    }