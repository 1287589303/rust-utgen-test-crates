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
fn script(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-script"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-script")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::script::BY_NAME;
        property_set(BY_NAME, name).map(hir_class).ok_or(Error::PropertyValueNotFound)
    }
    imp(canonical_name)
}
#[cfg(feature = "unicode-script")]
fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
    use crate::unicode_tables::script::BY_NAME;
    property_set(BY_NAME, name).map(hir_class).ok_or(Error::PropertyValueNotFound)
}
