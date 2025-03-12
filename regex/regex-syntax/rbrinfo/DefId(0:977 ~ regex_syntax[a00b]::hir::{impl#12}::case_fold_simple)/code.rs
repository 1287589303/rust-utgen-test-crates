pub fn case_fold_simple(&mut self) {
        self.set
            .case_fold_simple()
            .expect("unicode-case feature must be enabled");
    }