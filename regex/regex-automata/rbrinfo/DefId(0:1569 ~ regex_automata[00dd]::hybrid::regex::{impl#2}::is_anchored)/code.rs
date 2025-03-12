fn is_anchored(&self, input: &Input<'_>) -> bool {
        match input.get_anchored() {
            Anchored::No => {
                self.forward().get_nfa().is_always_start_anchored()
            }
            Anchored::Yes | Anchored::Pattern(_) => true,
        }
    }