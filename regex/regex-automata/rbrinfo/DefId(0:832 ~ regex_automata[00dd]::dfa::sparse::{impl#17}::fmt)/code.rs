fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StateIter").field("id", &self.id).finish()
    }