fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut wrote = false;
        if !self.slots().is_empty() {
            write!(f, "{:?}", self.slots())?;
            wrote = true;
        }
        if !self.looks().is_empty() {
            if wrote {
                write!(f, "/")?;
            }
            write!(f, "{:?}", self.looks())?;
            wrote = true;
        }
        if !wrote {
            write!(f, "N/A")?;
        }
        Ok(())
    }