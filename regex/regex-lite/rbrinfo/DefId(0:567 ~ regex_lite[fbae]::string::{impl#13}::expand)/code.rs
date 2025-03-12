pub fn expand(&self, replacement: &str, dst: &mut String) {
        interpolate::string(
            replacement,
            |index, dst| {
                let m = match self.get(index) {
                    None => return,
                    Some(m) => m,
                };
                dst.push_str(&self.haystack[m.range()]);
            },
            |name| self.pikevm.nfa().to_index(name),
            dst,
        );
    }