// Answer 0

#[test]
fn test_fmt_with_valid_input() {
    struct TestHashBuilder;

    let mut map = IndexMap {
        core: Default::default(),
        hash_builder: TestHashBuilder,
    };
    
    let entry_builder = RawEntryBuilderMut { map: &mut map };
    let formatter = &mut fmt::Formatter::new();
    
    entry_builder.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_with_empty_formatter() {
    struct TestHashBuilder;

    let mut map = IndexMap {
        core: Default::default(),
        hash_builder: TestHashBuilder,
    };
    
    let entry_builder = RawEntryBuilderMut { map: &mut map };
    let formatter = &mut fmt::Formatter::new().finish();
    
    entry_builder.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_with_reserved_formatter() {
    struct TestHashBuilder;

    let mut map = IndexMap {
        core: Default::default(),
        hash_builder: TestHashBuilder,
    };
    
    let entry_builder = RawEntryBuilderMut { map: &mut map };
    let formatter = &mut fmt::Formatter::new();
    
    entry_builder.fmt(formatter).unwrap();
}

