fn reset(&mut self, re: &PikeVM) {
        self.set.resize(re.nfa().len());
        self.slot_table.reset(re);
    }