pub fn interpolate_string(
        &self,
        haystack: &str,
        replacement: &str,
    ) -> String {
        let mut dst = String::new();
        self.interpolate_string_into(haystack, replacement, &mut dst);
        dst
    }