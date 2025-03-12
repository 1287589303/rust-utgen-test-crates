fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printed = false;
        for i in 0..(self.ntrans - 1) {
            let next = self.next_at(i);
            if next == DEAD {
                continue;
            }

            if printed {
                write!(f, ", ")?;
            }
            let (start, end) = self.range(i);
            if start == end {
                write!(f, "{:?} => {:?}", DebugByte(start), next.as_usize())?;
            } else {
                write!(
                    f,
                    "{:?}-{:?} => {:?}",
                    DebugByte(start),
                    DebugByte(end),
                    next.as_usize(),
                )?;
            }
            printed = true;
        }
        let eoi = self.next_at(self.ntrans - 1);
        if eoi != DEAD {
            if printed {
                write!(f, ", ")?;
            }
            write!(f, "EOI => {:?}", eoi.as_usize())?;
        }
        Ok(())
    }