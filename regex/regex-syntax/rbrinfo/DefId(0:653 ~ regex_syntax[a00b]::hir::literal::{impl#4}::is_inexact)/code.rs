pub fn is_inexact(&self) -> bool {
        self.literals().map_or(true, |lits| lits.iter().all(|x| !x.is_exact()))
    }