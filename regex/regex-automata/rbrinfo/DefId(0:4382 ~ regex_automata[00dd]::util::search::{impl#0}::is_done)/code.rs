pub fn is_done(&self) -> bool {
        self.get_span().start > self.get_span().end
    }