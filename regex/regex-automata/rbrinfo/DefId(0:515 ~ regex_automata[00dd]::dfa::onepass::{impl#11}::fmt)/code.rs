fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.is_dead() {
            return write!(f, "0");
        }
        write!(f, "{}", self.state_id().as_usize())?;
        if self.match_wins() {
            write!(f, "-MW")?;
        }
        if !self.epsilons().is_empty() {
            write!(f, "-{:?}", self.epsilons())?;
        }
        Ok(())
    }