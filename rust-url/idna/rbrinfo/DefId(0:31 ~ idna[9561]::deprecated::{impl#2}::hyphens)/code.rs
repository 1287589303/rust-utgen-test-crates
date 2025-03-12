fn hyphens(&self) -> Hyphens {
        if self.check_hyphens {
            Hyphens::CheckFirstLast
        } else {
            Hyphens::Allow
        }
    }