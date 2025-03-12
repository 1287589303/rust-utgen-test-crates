pub fn set_start(&mut self, start: usize) {
        self.set_span(Span { start, ..self.get_span() });
    }