fn log_violation_if(&self, v: SyntaxViolation, test: impl FnOnce() -> bool) {
        if let Some(f) = self.violation_fn {
            if test() {
                f(v)
            }
        }
    }