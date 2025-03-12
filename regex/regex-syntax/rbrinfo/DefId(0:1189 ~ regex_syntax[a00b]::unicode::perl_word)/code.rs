pub fn perl_word() -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-perl"))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        Err(Error::PerlClassNotFound)
    }

    #[cfg(feature = "unicode-perl")]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::perl_word::PERL_WORD;
        Ok(hir_class(PERL_WORD))
    }

    imp()
}