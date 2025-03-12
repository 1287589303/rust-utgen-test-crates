fn mutate<F: FnOnce(&mut Parser<'_>) -> R, R>(&mut self, f: F) -> R {
        let mut parser = Parser::for_setter(mem::take(&mut self.serialization));
        let result = f(&mut parser);
        self.serialization = parser.serialization;
        result
    }