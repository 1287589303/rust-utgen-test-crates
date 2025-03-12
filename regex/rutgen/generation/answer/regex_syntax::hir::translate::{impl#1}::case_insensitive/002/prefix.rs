// Answer 0

#[test]
fn test_case_insensitive_false() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.case_insensitive(false);
}

#[test]
fn test_case_insensitive_chain() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.case_insensitive(false).case_insensitive(false);
}

#[test]
fn test_case_insensitive_multiple_calls() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.case_insensitive(false)
        .case_insensitive(true)
        .case_insensitive(false);
}

