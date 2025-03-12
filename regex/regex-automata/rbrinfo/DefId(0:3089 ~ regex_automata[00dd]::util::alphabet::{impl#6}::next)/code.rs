fn next(&mut self) -> Option<Unit> {
        while self.cur_byte < self.end_byte.unwrap_or(256) {
            let byte = u8::try_from(self.cur_byte).unwrap();
            let class = self.classes.get(byte);
            self.cur_byte += 1;

            if self.last_class != Some(class) {
                self.last_class = Some(class);
                return Some(Unit::u8(byte));
            }
        }
        if self.cur_byte != usize::MAX && self.end_byte.is_none() {
            // Using usize::MAX as a sentinel is OK because we ban usize::MAX
            // from appearing as a start bound in iterator construction. But
            // why do it this way? Well, we want to return the EOI class
            // whenever the end of the given range is unbounded because EOI
            // isn't really a "byte" per se, so the only way it should be
            // excluded is if there is a bounded end to the range. Therefore,
            // when the end is unbounded, we just need to know whether we've
            // reported EOI or not. When we do, we set cur_byte to a value it
            // can never otherwise be.
            self.cur_byte = usize::MAX;
            return Some(self.classes.eoi());
        }
        None
    }