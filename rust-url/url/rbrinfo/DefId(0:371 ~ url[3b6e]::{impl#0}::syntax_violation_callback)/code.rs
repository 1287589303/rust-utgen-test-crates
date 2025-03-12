pub fn syntax_violation_callback(mut self, new: Option<&'a dyn Fn(SyntaxViolation)>) -> Self {
        self.violation_fn = new;
        self
    }