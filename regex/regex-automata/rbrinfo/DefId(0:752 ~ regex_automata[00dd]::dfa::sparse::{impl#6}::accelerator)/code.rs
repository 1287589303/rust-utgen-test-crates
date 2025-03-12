fn accelerator(&self, id: StateID) -> &[u8] {
        self.tt.state(id).accelerator()
    }