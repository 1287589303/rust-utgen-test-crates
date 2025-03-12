fn ascii_case_fold(&mut self) {
        let len = self.ranges.len();
        for i in 0..len {
            if let Some(folded) = self.ranges[i].ascii_case_fold() {
                self.ranges.push(folded);
            }
        }
        self.canonicalize();
    }