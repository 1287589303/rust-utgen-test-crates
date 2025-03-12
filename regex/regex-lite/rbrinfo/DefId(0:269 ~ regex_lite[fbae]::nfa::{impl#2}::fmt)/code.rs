fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "NFA(")?;
        writeln!(f, "pattern: {}", self.pattern)?;
        for (sid, state) in self.states.iter().enumerate() {
            writeln!(f, "{:07?}: {:?}", sid, state)?;
        }
        writeln!(f, ")")?;
        Ok(())
    }