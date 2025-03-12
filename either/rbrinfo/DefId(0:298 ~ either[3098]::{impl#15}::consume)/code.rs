fn consume(&mut self, amt: usize) {
        for_both!(self, inner => inner.consume(amt))
    }