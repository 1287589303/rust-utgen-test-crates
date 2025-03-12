fn iter(&self) -> IterAccels<'_, A> {
        IterAccels { accels: self, i: 0 }
    }