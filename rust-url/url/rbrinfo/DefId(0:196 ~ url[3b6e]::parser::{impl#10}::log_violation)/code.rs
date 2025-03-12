fn log_violation(&self, v: SyntaxViolation) {
        if let Some(f) = self.violation_fn {
            f(v)
        }
    }