pub fn try_case_fold_simple(
        &mut self,
    ) -> core::result::Result<(), CaseFoldError> {
        match *self {
            Class::Unicode(ref mut x) => x.try_case_fold_simple()?,
            Class::Bytes(ref mut x) => x.case_fold_simple(),
        }
        Ok(())
    }