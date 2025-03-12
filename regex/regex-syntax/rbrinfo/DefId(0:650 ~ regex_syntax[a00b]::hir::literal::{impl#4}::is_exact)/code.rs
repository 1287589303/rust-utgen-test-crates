pub fn is_exact(&self) -> bool {
        self.literals().map_or(false, |lits| lits.iter().all(|x| x.is_exact()))
    }