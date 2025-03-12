pub fn reset(&mut self, start: char, end: char) {
        self.range_stack.clear();
        self.push(u32::from(start), u32::from(end));
    }