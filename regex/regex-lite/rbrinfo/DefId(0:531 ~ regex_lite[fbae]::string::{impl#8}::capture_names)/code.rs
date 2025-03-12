pub fn capture_names(&self) -> CaptureNames<'_> {
        CaptureNames(self.pikevm.nfa().capture_names())
    }