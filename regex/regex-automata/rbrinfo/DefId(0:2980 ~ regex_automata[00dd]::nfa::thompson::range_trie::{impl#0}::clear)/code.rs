pub fn clear(&mut self) {
        self.free.extend(self.states.drain(..));
        self.add_empty(); // final
        self.add_empty(); // root
    }