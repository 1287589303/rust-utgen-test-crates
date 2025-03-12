fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        fn debug_state_transitions(
            f: &mut core::fmt::Formatter,
            dfa: &DFA,
            sid: StateID,
        ) -> core::fmt::Result {
            for (i, (start, end, trans)) in
                dfa.sparse_transitions(sid).enumerate()
            {
                let next = trans.state_id();
                if i > 0 {
                    write!(f, ", ")?;
                }
                if start == end {
                    write!(
                        f,
                        "{:?} => {:?}",
                        DebugByte(start),
                        next.as_usize(),
                    )?;
                } else {
                    write!(
                        f,
                        "{:?}-{:?} => {:?}",
                        DebugByte(start),
                        DebugByte(end),
                        next.as_usize(),
                    )?;
                }
                if trans.match_wins() {
                    write!(f, " (MW)")?;
                }
                if !trans.epsilons().is_empty() {
                    write!(f, " ({:?})", trans.epsilons())?;
                }
            }
            Ok(())
        }

        writeln!(f, "onepass::DFA(")?;
        for index in 0..self.state_len() {
            let sid = StateID::must(index);
            let pateps = self.pattern_epsilons(sid);
            if sid == DEAD {
                write!(f, "D ")?;
            } else if pateps.pattern_id().is_some() {
                write!(f, "* ")?;
            } else {
                write!(f, "  ")?;
            }
            write!(f, "{:06?}", sid.as_usize())?;
            if !pateps.is_empty() {
                write!(f, " ({:?})", pateps)?;
            }
            write!(f, ": ")?;
            debug_state_transitions(f, self, sid)?;
            write!(f, "\n")?;
        }
        writeln!(f, "")?;
        for (i, &sid) in self.starts.iter().enumerate() {
            if i == 0 {
                writeln!(f, "START(ALL): {:?}", sid.as_usize())?;
            } else {
                writeln!(
                    f,
                    "START(pattern: {:?}): {:?}",
                    i - 1,
                    sid.as_usize(),
                )?;
            }
        }
        writeln!(f, "state length: {:?}", self.state_len())?;
        writeln!(f, "pattern length: {:?}", self.pattern_len())?;
        writeln!(f, ")")?;
        Ok(())
    }