fn reset(&mut self, re: &PikeVM) {
        self.set.resize(re.get_nfa().states().len());
        self.slot_table.reset(re);
    }