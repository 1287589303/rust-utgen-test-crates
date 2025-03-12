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
fn ages(canonical_age: &str) -> Result<impl Iterator<Item = Range>, Error> {
    #[cfg(not(feature = "unicode-age"))]
    fn imp(_: &str) -> Result<impl Iterator<Item = Range>, Error> {
        use core::option::IntoIter;
        Err::<IntoIter<Range>, _>(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-age")]
    fn imp(canonical_age: &str) -> Result<impl Iterator<Item = Range>, Error> {
        use crate::unicode_tables::age;
        const AGES: &[(&str, Range)] = &[
            ("V1_1", age::V1_1),
            ("V2_0", age::V2_0),
            ("V2_1", age::V2_1),
            ("V3_0", age::V3_0),
            ("V3_1", age::V3_1),
            ("V3_2", age::V3_2),
            ("V4_0", age::V4_0),
            ("V4_1", age::V4_1),
            ("V5_0", age::V5_0),
            ("V5_1", age::V5_1),
            ("V5_2", age::V5_2),
            ("V6_0", age::V6_0),
            ("V6_1", age::V6_1),
            ("V6_2", age::V6_2),
            ("V6_3", age::V6_3),
            ("V7_0", age::V7_0),
            ("V8_0", age::V8_0),
            ("V9_0", age::V9_0),
            ("V10_0", age::V10_0),
            ("V11_0", age::V11_0),
            ("V12_0", age::V12_0),
            ("V12_1", age::V12_1),
            ("V13_0", age::V13_0),
            ("V14_0", age::V14_0),
            ("V15_0", age::V15_0),
            ("V15_1", age::V15_1),
            ("V16_0", age::V16_0),
        ];
        assert_eq!(AGES.len(), age::BY_NAME.len(), "ages are out of sync");
        let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
        match pos {
            None => Err(Error::PropertyValueNotFound),
            Some(i) => Ok(AGES[..=i].iter().map(|&(_, classes)| classes)),
        }
    }
    imp(canonical_age)
}
#[cfg(feature = "unicode-age")]
fn imp(canonical_age: &str) -> Result<impl Iterator<Item = Range>, Error> {
    use crate::unicode_tables::age;
    const AGES: &[(&str, Range)] = &[
        ("V1_1", age::V1_1),
        ("V2_0", age::V2_0),
        ("V2_1", age::V2_1),
        ("V3_0", age::V3_0),
        ("V3_1", age::V3_1),
        ("V3_2", age::V3_2),
        ("V4_0", age::V4_0),
        ("V4_1", age::V4_1),
        ("V5_0", age::V5_0),
        ("V5_1", age::V5_1),
        ("V5_2", age::V5_2),
        ("V6_0", age::V6_0),
        ("V6_1", age::V6_1),
        ("V6_2", age::V6_2),
        ("V6_3", age::V6_3),
        ("V7_0", age::V7_0),
        ("V8_0", age::V8_0),
        ("V9_0", age::V9_0),
        ("V10_0", age::V10_0),
        ("V11_0", age::V11_0),
        ("V12_0", age::V12_0),
        ("V12_1", age::V12_1),
        ("V13_0", age::V13_0),
        ("V14_0", age::V14_0),
        ("V15_0", age::V15_0),
        ("V15_1", age::V15_1),
        ("V16_0", age::V16_0),
    ];
    assert_eq!(AGES.len(), age::BY_NAME.len(), "ages are out of sync");
    let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
    match pos {
        None => Err(Error::PropertyValueNotFound),
        Some(i) => Ok(AGES[..=i].iter().map(|&(_, classes)| classes)),
    }
}
