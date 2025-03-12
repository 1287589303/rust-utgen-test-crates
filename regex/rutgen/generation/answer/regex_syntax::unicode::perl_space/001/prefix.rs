// Answer 0

#[test]
fn test_perl_space_no_features() {
    // No feature flags set; expect Error::PerlClassNotFound
    let result = perl_space();
}

#[test]
#[cfg(feature = "unicode-perl")]
fn test_perl_space_unicode_perl_feature() {
    // Feature "unicode-perl" enabled; expect success with valid ClassUnicode
    let result = perl_space();
}

#[test]
#[cfg(feature = "unicode-bool")]
fn test_perl_space_unicode_bool_feature() {
    // Feature "unicode-bool" enabled; expect success with valid ClassUnicode
    let result = perl_space();
}

#[test]
#[cfg(all(feature = "unicode-perl", feature = "unicode-bool"))]
fn test_perl_space_both_features() {
    // Both feature flags enabled; expect success with valid ClassUnicode
    let result = perl_space();
}

