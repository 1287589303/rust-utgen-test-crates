fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            State::ByteRange { ref trans } => trans.fmt(f),
            State::Sparse(SparseTransitions { ref transitions }) => {
                let rs = transitions
                    .iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "sparse({})", rs)
            }
            State::Dense(ref dense) => {
                write!(f, "dense(")?;
                for (i, t) in dense.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", t)?;
                }
                write!(f, ")")
            }
            State::Look { ref look, next } => {
                write!(f, "{:?} => {:?}", look, next.as_usize())
            }
            State::Union { ref alternates } => {
                let alts = alternates
                    .iter()
                    .map(|id| format!("{:?}", id.as_usize()))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "union({})", alts)
            }
            State::BinaryUnion { alt1, alt2 } => {
                write!(
                    f,
                    "binary-union({}, {})",
                    alt1.as_usize(),
                    alt2.as_usize()
                )
            }
            State::Capture { next, pattern_id, group_index, slot } => {
                write!(
                    f,
                    "capture(pid={:?}, group={:?}, slot={:?}) => {:?}",
                    pattern_id.as_usize(),
                    group_index.as_usize(),
                    slot.as_usize(),
                    next.as_usize(),
                )
            }
            State::Fail => write!(f, "FAIL"),
            State::Match { pattern_id } => {
                write!(f, "MATCH({:?})", pattern_id.as_usize())
            }
        }
    }