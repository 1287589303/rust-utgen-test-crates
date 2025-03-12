fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        for (i, state) in self.states.iter().enumerate() {
            let status = if i == FINAL.as_usize() { '*' } else { ' ' };
            writeln!(f, "{}{:06}: {:?}", status, i, state)?;
        }
        Ok(())
    }