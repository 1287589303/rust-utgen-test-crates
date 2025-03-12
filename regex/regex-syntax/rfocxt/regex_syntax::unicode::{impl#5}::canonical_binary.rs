type Range = &'static [(char, char)];
type PropertyValues = &'static [(&'static str, &'static str)];
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use crate::hir;
#[derive(Debug)]
pub enum ClassQuery<'a> {
    /// Return a class corresponding to a Unicode binary property, named by
    /// a single letter.
    OneLetter(char),
    /// Return a class corresponding to a Unicode binary property.
    ///
    /// Note that, by special exception (see UTS#18, Section 1.2), both
    /// general category values and script values are permitted here as if
    /// they were a binary property.
    Binary(&'a str),
    /// Return a class corresponding to all codepoints whose property
    /// (identified by `property_name`) corresponds to the given value
    /// (identified by `property_value`).
    ByValue {
        /// A property name.
        property_name: &'a str,
        /// A property value.
        property_value: &'a str,
    },
}
#[derive(Debug, Eq, PartialEq)]
enum CanonicalClassQuery {
    /// The canonical binary property name.
    Binary(&'static str),
    /// The canonical general category name.
    GeneralCategory(&'static str),
    /// The canonical script name.
    Script(&'static str),
    /// An arbitrary association between property and value, both of which
    /// have been canonicalized.
    ///
    /// Note that by construction, the property name of ByValue will never
    /// be General_Category or Script. Those two cases are subsumed by the
    /// eponymous variants.
    ByValue {
        /// The canonical property name.
        property_name: &'static str,
        /// The canonical property value.
        property_value: &'static str,
    },
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
    #[allow(dead_code)]
    PerlClassNotFound,
}
impl<'a> ClassQuery<'a> {
    fn canonicalize(&self) -> Result<CanonicalClassQuery, Error> {}
    fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery, Error> {
        let norm = symbolic_name_normalize(name);
        if norm != "cf" && norm != "sc" && norm != "lc" {
            if let Some(canon) = canonical_prop(&norm)? {
                return Ok(CanonicalClassQuery::Binary(canon));
            }
        }
        if let Some(canon) = canonical_gencat(&norm)? {
            return Ok(CanonicalClassQuery::GeneralCategory(canon));
        }
        if let Some(canon) = canonical_script(&norm)? {
            return Ok(CanonicalClassQuery::Script(canon));
        }
        Err(Error::PropertyNotFound)
    }
}
fn symbolic_name_normalize(x: &str) -> String {
    let mut tmp = x.as_bytes().to_vec();
    let len = symbolic_name_normalize_bytes(&mut tmp).len();
    tmp.truncate(len);
    String::from_utf8(tmp).unwrap()
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
fn canonical_script(normalized_value: &str) -> Result<Option<&'static str>, Error> {
    let scripts = property_values("Script")?.unwrap();
    Ok(canonical_value(scripts, normalized_value))
}
fn canonical_gencat(normalized_value: &str) -> Result<Option<&'static str>, Error> {
    Ok(
        match normalized_value {
            "any" => Some("Any"),
            "assigned" => Some("Assigned"),
            "ascii" => Some("ASCII"),
            _ => {
                let gencats = property_values("General_Category")?.unwrap();
                canonical_value(gencats, normalized_value)
            }
        },
    )
}
