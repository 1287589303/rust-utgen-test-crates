pub fn new() -> Result<SimpleCaseFolder, CaseFoldError> {
        #[cfg(not(feature = "unicode-case"))]
        {
            Err(CaseFoldError(()))
        }
        #[cfg(feature = "unicode-case")]
        {
            Ok(SimpleCaseFolder {
                table: crate::unicode_tables::case_folding_simple::CASE_FOLDING_SIMPLE,
                last: None,
                next: 0,
            })
        }
    }