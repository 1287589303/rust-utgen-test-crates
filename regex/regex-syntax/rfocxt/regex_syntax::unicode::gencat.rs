type Range = &'static [(char, char)];
type PropertyValues = &'static [(&'static str, &'static str)];
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use crate::hir;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
    #[allow(dead_code)]
    PerlClassNotFound,
}
fn gencat(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-gencat"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-gencat")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::general_category::BY_NAME;
        match name {
            "ASCII" => Ok(hir_class(&[('\0', '\x7F')])),
            "Any" => Ok(hir_class(&[('\0', '\u{10FFFF}')])),
            "Assigned" => {
                let mut cls = gencat("Unassigned")?;
                cls.negate();
                Ok(cls)
            }
            name => {
                property_set(BY_NAME, name)
                    .map(hir_class)
                    .ok_or(Error::PropertyValueNotFound)
            }
        }
    }
    match canonical_name {
        "Decimal_Number" => perl_digit(),
        name => imp(name),
    }
}
#[cfg(feature = "unicode-gencat")]
fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
    use crate::unicode_tables::general_category::BY_NAME;
    match name {
        "ASCII" => Ok(hir_class(&[('\0', '\x7F')])),
        "Any" => Ok(hir_class(&[('\0', '\u{10FFFF}')])),
        "Assigned" => {
            let mut cls = gencat("Unassigned")?;
            cls.negate();
            Ok(cls)
        }
        name => {
            property_set(BY_NAME, name)
                .map(hir_class)
                .ok_or(Error::PropertyValueNotFound)
        }
    }
}
pub fn perl_digit() -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(any(feature = "unicode-perl", feature = "unicode-gencat")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        Err(Error::PerlClassNotFound)
    }
    #[cfg(all(feature = "unicode-perl", not(feature = "unicode-gencat")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::perl_decimal::DECIMAL_NUMBER;
        Ok(hir_class(DECIMAL_NUMBER))
    }
    #[cfg(feature = "unicode-gencat")]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::general_category::DECIMAL_NUMBER;
        Ok(hir_class(DECIMAL_NUMBER))
    }
    imp()
}
