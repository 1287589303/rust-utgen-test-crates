fn starts_with<P: Pattern>(&self, p: P) -> bool {
        p.split_prefix(&mut self.clone())
    }