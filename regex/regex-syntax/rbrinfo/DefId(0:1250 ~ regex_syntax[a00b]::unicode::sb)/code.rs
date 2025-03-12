fn sb(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-segment"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }

    #[cfg(feature = "unicode-segment")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::sentence_break::BY_NAME;
        property_set(BY_NAME, name)
            .map(hir_class)
            .ok_or(Error::PropertyValueNotFound)
    }

    imp(canonical_name)
}