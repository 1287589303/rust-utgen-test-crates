fn set_true<A: ToTokens>(&mut self, obj: A) {
        self.0.set(obj, ());
    }