pub fn expand(&self, replacement: &str, dst: &mut String) {
        self.caps.interpolate_string_into(self.haystack, replacement, dst);
    }