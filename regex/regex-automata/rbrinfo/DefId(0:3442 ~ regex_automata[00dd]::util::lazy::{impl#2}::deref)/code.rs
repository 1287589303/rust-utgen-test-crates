fn deref(&self) -> &T {
        Lazy::get(self)
    }