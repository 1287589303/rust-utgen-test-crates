fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "S")?;
        for slot in self.iter() {
            write!(f, "-{:?}", slot)?;
        }
        Ok(())
    }