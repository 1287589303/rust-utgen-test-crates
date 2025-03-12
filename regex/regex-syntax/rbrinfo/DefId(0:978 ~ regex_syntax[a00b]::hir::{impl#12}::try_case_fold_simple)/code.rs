pub fn try_case_fold_simple(
        &mut self,
    ) -> core::result::Result<(), CaseFoldError> {
        self.set.case_fold_simple()
    }