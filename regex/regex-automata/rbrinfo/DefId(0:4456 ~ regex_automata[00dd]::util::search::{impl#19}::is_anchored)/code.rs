pub fn is_anchored(&self) -> bool {
        matches!(*self, Anchored::Yes | Anchored::Pattern(_))
    }