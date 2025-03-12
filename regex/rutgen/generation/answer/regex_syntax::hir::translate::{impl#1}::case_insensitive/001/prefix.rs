// Answer 0

#[test]
fn test_case_insensitive_true() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.case_insensitive(true);
}

#[test]
fn test_case_insensitive_false() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.case_insensitive(false);
}

