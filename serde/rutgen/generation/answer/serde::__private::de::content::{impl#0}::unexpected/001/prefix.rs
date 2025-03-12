// Answer 0

#[test]
fn test_unexpected_with_map() {
    let map_content = Content::Map(vec![
        (Content::String("key1".to_string()), Content::Bool(true)),
        (Content::U32(42), Content::F64(3.14)),
    ]);
    let _result = map_content.unexpected();
}

#[test]
fn test_unexpected_with_empty_map() {
    let map_content = Content::Map(vec![]);
    let _result = map_content.unexpected();
}

#[test]
fn test_unexpected_with_map_with_different_types() {
    let map_content = Content::Map(vec![
        (Content::I8(-1), Content::U16(100)),
        (Content::Char('a'), Content::String("value".to_string())),
    ]);
    let _result = map_content.unexpected();
}

