fn is_anchored(&self, input: &Input<'_>) -> bool {
        match input.get_anchored() {
            Anchored::No => self.forward().is_always_start_anchored(),
            Anchored::Yes | Anchored::Pattern(_) => true,
        }
    }