fn decrement_depth(&self) {
        let old = self.depth.get();
        // If this fails then the caller has a bug in how they're incrementing
        // and decrementing the depth of the parser's call stack.
        let new = old.checked_sub(1).unwrap();
        self.depth.set(new);
    }