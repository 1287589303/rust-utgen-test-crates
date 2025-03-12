fn next(&mut self) -> Option<State<'a>> {
        self.it.next().map(|(index, _)| {
            let id = self.tt.to_state_id(index);
            self.tt.state(id)
        })
    }