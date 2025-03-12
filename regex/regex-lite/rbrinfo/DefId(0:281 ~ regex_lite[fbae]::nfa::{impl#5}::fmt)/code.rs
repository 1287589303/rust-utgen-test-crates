fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            State::Char { target, ch } => {
                write!(f, "{:?} => {:?}", ch, target)
            }
            State::Ranges { target, ref ranges } => {
                for (i, &(start, end)) in ranges.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}-{:?} => {:?}", start, end, target)?;
                }
                Ok(())
            }
            State::Splits { ref targets, reverse } => {
                write!(f, "splits(")?;
                for (i, sid) in
                    State::iter_splits(targets, reverse).enumerate()
                {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", sid)?;
                }
                write!(f, ")")
            }
            State::Goto { target, look: None } => {
                write!(f, "goto({:?})", target)
            }
            State::Goto { target, look: Some(look) } => {
                write!(f, "{:?} => {:?}", look, target)
            }
            State::Capture { target, slot } => {
                write!(f, "capture(slot={:?}) => {:?}", slot, target,)
            }
            State::Fail => write!(f, "FAIL"),
            State::Match => {
                write!(f, "MATCH")
            }
        }
    }