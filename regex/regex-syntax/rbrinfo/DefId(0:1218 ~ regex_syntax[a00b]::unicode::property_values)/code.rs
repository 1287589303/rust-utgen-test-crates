fn property_values(
    canonical_property_name: &'static str,
) -> Result<Option<PropertyValues>, Error> {
    #[cfg(not(any(
        feature = "unicode-age",
        feature = "unicode-bool",
        feature = "unicode-gencat",
        feature = "unicode-perl",
        feature = "unicode-script",
        feature = "unicode-segment",
    )))]
    fn imp(_: &'static str) -> Result<Option<PropertyValues>, Error> {
        Err(Error::PropertyValueNotFound)
    }

    #[cfg(any(
        feature = "unicode-age",
        feature = "unicode-bool",
        feature = "unicode-gencat",
        feature = "unicode-perl",
        feature = "unicode-script",
        feature = "unicode-segment",
    ))]
    fn imp(name: &'static str) -> Result<Option<PropertyValues>, Error> {
        use crate::unicode_tables::property_values::PROPERTY_VALUES;

        Ok(PROPERTY_VALUES
            .binary_search_by_key(&name, |&(n, _)| n)
            .ok()
            .map(|i| PROPERTY_VALUES[i].1))
    }

    imp(canonical_property_name)
}