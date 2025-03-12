fn bump_if(&self, prefix: &str) -> bool {
        if self.pattern()[self.pos()..].starts_with(prefix) {
            for _ in 0..prefix.chars().count() {
                self.bump();
            }
            true
        } else {
            false
        }
    }