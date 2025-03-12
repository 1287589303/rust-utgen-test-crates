fn add_look(&self, mut look: Look) -> Result<StateID, BuildError> {
        if self.is_reverse() {
            look = look.reversed();
        }
        self.builder.borrow_mut().add_look(StateID::ZERO, look)
    }