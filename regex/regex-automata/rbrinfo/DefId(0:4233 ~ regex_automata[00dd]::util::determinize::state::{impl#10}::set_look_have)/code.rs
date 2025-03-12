fn set_look_have(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {
        set(self.look_have()).write_repr(&mut self.0[1..]);
    }