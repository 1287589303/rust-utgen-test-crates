fn bool_property(
    canonical_name: &'static str,
) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-bool"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }

    #[cfg(feature = "unicode-bool")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::property_bool::BY_NAME;
        property_set(BY_NAME, name)
            .map(hir_class)
            .ok_or(Error::PropertyNotFound)
    }

    match canonical_name {
        "Decimal_Number" => perl_digit(),
        "White_Space" => perl_space(),
        name => imp(name),
    }
}