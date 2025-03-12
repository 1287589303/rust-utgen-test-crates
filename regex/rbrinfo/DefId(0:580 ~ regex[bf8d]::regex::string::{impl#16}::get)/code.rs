pub fn get(&self, i: usize) -> Option<(usize, usize)> {
        self.0.get_group(i).map(|sp| (sp.start, sp.end))
    }