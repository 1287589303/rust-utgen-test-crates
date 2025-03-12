fn unclosed_class_error(&self) -> ast::Error {
        for state in self.parser().stack_class.borrow().iter().rev() {
            if let ClassState::Open { ref set, .. } = *state {
                return self.error(set.span, ast::ErrorKind::ClassUnclosed);
            }
        }
        // We are guaranteed to have a non-empty stack with at least
        // one open bracket, so we should never get here.
        panic!("no open character class found")
    }