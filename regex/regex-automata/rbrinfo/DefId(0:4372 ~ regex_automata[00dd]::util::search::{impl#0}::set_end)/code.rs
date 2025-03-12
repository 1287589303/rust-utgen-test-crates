pub fn set_end(&mut self, end: usize) {
        self.set_span(Span { end, ..self.get_span() });
    }