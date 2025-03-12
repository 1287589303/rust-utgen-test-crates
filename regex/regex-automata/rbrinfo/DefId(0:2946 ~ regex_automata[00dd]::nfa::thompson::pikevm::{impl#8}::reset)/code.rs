pub fn reset(&mut self, re: &PikeVM) {
        self.curr.reset(re);
        self.next.reset(re);
    }