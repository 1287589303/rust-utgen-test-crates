// Answer 0

#[test]
fn test_into_iter_empty_string() {
    let wrapper = StringWrapper("");
    let chars: Vec<_> = wrapper.into_iter().collect();
    assert_eq!(chars, Vec::<char>::new());
}

#[test]
fn test_into_iter_single_character() {
    let wrapper = StringWrapper("A");
    let chars: Vec<_> = wrapper.into_iter().collect();
    assert_eq!(chars, vec!['A']);
}

#[test]
fn test_into_iter_multiple_characters() {
    let wrapper = StringWrapper("Hello");
    let chars: Vec<_> = wrapper.into_iter().collect();
    assert_eq!(chars, vec!['H', 'e', 'l', 'l', 'o']);
}

#[test]
fn test_into_iter_whitespace() {
    let wrapper = StringWrapper(" \t\n");
    let chars: Vec<_> = wrapper.into_iter().collect();
    assert_eq!(chars, vec![' ', '\t', '\n']);
}

#[test]
fn test_into_iter_unicode_characters() {
    let wrapper = StringWrapper("こんにちは");
    let chars: Vec<_> = wrapper.into_iter().collect();
    assert_eq!(chars, vec!['こ', 'ん', 'に', 'ち', 'は']);
}

