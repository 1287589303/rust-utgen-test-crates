fn is_accelerated(&self) -> bool {
        self.pre.as_ref().map_or(false, |pre| pre.is_fast())
    }