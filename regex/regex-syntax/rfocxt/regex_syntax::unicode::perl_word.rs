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
#[cfg(feature = "unicode-perl")]
fn imp() -> Result<hir::ClassUnicode, Error> {
    use crate::unicode_tables::perl_word::PERL_WORD;
    Ok(hir_class(PERL_WORD))
}
