fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.to_string().hash(hasher);
    }