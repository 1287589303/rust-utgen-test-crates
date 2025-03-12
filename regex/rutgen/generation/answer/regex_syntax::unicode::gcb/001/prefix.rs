// Answer 0

#[test]
fn test_gcb_valid_property() {
    let result = gcb("Extend");
}

#[test]
fn test_gcb_non_existing_property() {
    let result = gcb("NonExistingProperty");
}

#[cfg(feature = "unicode-segment")]
#[test]
fn test_gcb_feature_property() {
    let result = gcb("CR");
}

#[cfg(not(feature = "unicode-segment"))]
#[test]
fn test_gcb_property_not_found() {
    let result = gcb("CR");
}

