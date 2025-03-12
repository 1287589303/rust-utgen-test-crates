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
    fn canonicalize(&self) -> Result<CanonicalClassQuery, Error> {
        match *self {
            ClassQuery::OneLetter(c) => self.canonical_binary(&c.to_string()),
            ClassQuery::Binary(name) => self.canonical_binary(name),
            ClassQuery::ByValue { property_name, property_value } => {
                let property_name = symbolic_name_normalize(property_name);
                let property_value = symbolic_name_normalize(property_value);
                let canon_name = match canonical_prop(&property_name)? {
                    None => return Err(Error::PropertyNotFound),
                    Some(canon_name) => canon_name,
                };
                Ok(
                    match canon_name {
                        "General_Category" => {
                            let canon = match canonical_gencat(&property_value)? {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(canon) => canon,
                            };
                            CanonicalClassQuery::GeneralCategory(canon)
                        }
                        "Script" => {
                            let canon = match canonical_script(&property_value)? {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(canon) => canon,
                            };
                            CanonicalClassQuery::Script(canon)
                        }
                        _ => {
                            let vals = match property_values(canon_name)? {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(vals) => vals,
                            };
                            let canon_val = match canonical_value(
                                vals,
                                &property_value,
                            ) {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(canon_val) => canon_val,
                            };
                            CanonicalClassQuery::ByValue {
                                property_name: canon_name,
                                property_value: canon_val,
                            }
                        }
                    },
                )
            }
        }
    }
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
fn property_values(
    canonical_property_name: &'static str,
) -> Result<Option<PropertyValues>, Error> {
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
    fn imp(_: &'static str) -> Result<Option<PropertyValues>, Error> {
        Err(Error::PropertyValueNotFound)
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
    fn imp(name: &'static str) -> Result<Option<PropertyValues>, Error> {
        use crate::unicode_tables::property_values::PROPERTY_VALUES;
        Ok(
            PROPERTY_VALUES
                .binary_search_by_key(&name, |&(n, _)| n)
                .ok()
                .map(|i| PROPERTY_VALUES[i].1),
        )
    }
    imp(canonical_property_name)
}
fn canonical_value(
    vals: PropertyValues,
    normalized_value: &str,
) -> Option<&'static str> {
    vals.binary_search_by_key(&normalized_value, |&(n, _)| n).ok().map(|i| vals[i].1)
}
fn symbolic_name_normalize(x: &str) -> String {
    let mut tmp = x.as_bytes().to_vec();
    let len = symbolic_name_normalize_bytes(&mut tmp).len();
    tmp.truncate(len);
    String::from_utf8(tmp).unwrap()
}
