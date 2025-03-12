fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeln!(f, "LiteralTrie(")?;
        for (sid, state) in self.states.iter().with_state_ids() {
            writeln!(f, "{:06?}: {:?}", sid.as_usize(), state)?;
        }
        writeln!(f, ")")?;
        Ok(())
    }