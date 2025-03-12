fn check_url_code_point(&self, c: char, input: &Input<'_>) {
        if let Some(vfn) = self.violation_fn {
            check_url_code_point(vfn, c, input)
        }
    }