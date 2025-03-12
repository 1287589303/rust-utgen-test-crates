fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Seq")?;
        if let Some(lits) = self.literals() {
            f.debug_list().entries(lits.iter()).finish()
        } else {
            write!(f, "[∞]")
        }
    }