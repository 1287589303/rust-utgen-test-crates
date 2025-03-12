fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StartStateIter").field("i", &self.i).finish()
    }