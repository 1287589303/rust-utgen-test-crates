pub(crate) fn to_index(&self, name: &str) -> Option<usize> {
        self.cap_name_to_index.get(name).cloned().map(|i| i.as_usize())
    }