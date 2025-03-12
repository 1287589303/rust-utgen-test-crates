pub(crate) fn to_state(&self) -> State {
        State(Arc::from(&*self.repr))
    }