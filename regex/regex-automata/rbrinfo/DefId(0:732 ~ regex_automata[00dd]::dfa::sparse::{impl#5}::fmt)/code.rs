fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "sparse::DFA(")?;
        for state in self.tt.states() {
            fmt_state_indicator(f, self, state.id())?;
            writeln!(f, "{:06?}: {:?}", state.id().as_usize(), state)?;
        }
        writeln!(f, "")?;
        for (i, (start_id, anchored, sty)) in self.st.iter().enumerate() {
            if i % self.st.stride == 0 {
                match anchored {
                    Anchored::No => writeln!(f, "START-GROUP(unanchored)")?,
                    Anchored::Yes => writeln!(f, "START-GROUP(anchored)")?,
                    Anchored::Pattern(pid) => writeln!(
                        f,
                        "START_GROUP(pattern: {:?})",
                        pid.as_usize()
                    )?,
                }
            }
            writeln!(f, "  {:?} => {:06?}", sty, start_id.as_usize())?;
        }
        writeln!(f, "state length: {:?}", self.tt.state_len)?;
        writeln!(f, "pattern length: {:?}", self.pattern_len())?;
        writeln!(f, "flags: {:?}", self.flags)?;
        writeln!(f, ")")?;
        Ok(())
    }