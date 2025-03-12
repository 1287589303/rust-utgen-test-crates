// Answer 0

#[test]
fn test_from_name_lower() {
    let result = ClassAsciiKind::from_name("lower");
    // The expected return value is Some(ClassAsciiKind::Lower)
    let _ = result;
}

#[test]
fn test_from_name_non_matching() {
    let names = [
        "alnum",
        "alpha",
        "ascii",
        "blank",
        "cntrl",
        "digit",
        "graph",
    ];
    for &name in &names {
        let result = ClassAsciiKind::from_name(name);
        // The expected return value is None
        let _ = result;
    }
}

