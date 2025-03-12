fn set_look_need(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {
        set(self.look_need()).write_repr(&mut self.0[5..]);
    }