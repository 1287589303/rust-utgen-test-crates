pub fn captures(self, yes: bool) -> Config {
        self.which_captures(if yes {
            WhichCaptures::All
        } else {
            WhichCaptures::None
        })
    }