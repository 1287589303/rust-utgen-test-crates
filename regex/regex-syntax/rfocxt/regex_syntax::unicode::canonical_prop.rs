type Range = &'static [(char, char)];
type PropertyValues = &'static [(&'static str, &'static str)];
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use crate::hir;
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
    #[allow(dead_code)]
    PerlClassNotFound,
}
fn canonical_prop(normalized_name: &str) -> Result<Option<&'static str>, Error> {
    #[cfg(
        not(
            any(
                feature = "unicode-age",
                feature = "unicode-bool",
                feature = "unicode-gencat",
                feature = "unicode-perl",
                feature = "unicode-script",
                feature = "unicode-segment",
            )
        )
    )]
    fn imp(_: &str) -> Result<Option<&'static str>, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(
        any(
            feature = "unicode-age",
            feature = "unicode-bool",
            feature = "unicode-gencat",
            feature = "unicode-perl",
            feature = "unicode-script",
            feature = "unicode-segment",
        )
    )]
    fn imp(name: &str) -> Result<Option<&'static str>, Error> {
        use crate::unicode_tables::property_names::PROPERTY_NAMES;
        Ok(
            PROPERTY_NAMES
                .binary_search_by_key(&name, |&(n, _)| n)
                .ok()
                .map(|i| PROPERTY_NAMES[i].1),
        )
    }
    imp(normalized_name)
}
#[cfg(
    any(
        feature = "unicode-age",
        feature = "unicode-bool",
        feature = "unicode-gencat",
        feature = "unicode-perl",
        feature = "unicode-script",
        feature = "unicode-segment",
    )
)]
fn imp(name: &str) -> Result<Option<&'static str>, Error> {
    use crate::unicode_tables::property_names::PROPERTY_NAMES;
    Ok(
        PROPERTY_NAMES
            .binary_search_by_key(&name, |&(n, _)| n)
            .ok()
            .map(|i| PROPERTY_NAMES[i].1),
    )
}
