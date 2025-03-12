pub fn get_match(&self) -> Option<Match> {
        Some(Match::new(self.pattern()?, self.get_group(0)?))
    }