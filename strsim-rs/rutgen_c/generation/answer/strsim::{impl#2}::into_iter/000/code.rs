// Answer 0

#[test]
fn test_into_iter() {
    let wrapper = StringWrapper("test");
    let collected_chars: Vec<char> = wrapper.into_iter().collect();
    assert_eq!(collected_chars, vec!['t', 'e', 's', 't']);
}

#[test]
fn test_into_iter_empty_string() {
    let wrapper = StringWrapper("");
    let collected_chars: Vec<char> = wrapper.into_iter().collect();
    assert_eq!(collected_chars, vec![]);
}

#[test]
fn test_into_iter_single_char() {
    let wrapper = StringWrapper("a");
    let collected_chars: Vec<char> = wrapper.into_iter().collect();
    assert_eq!(collected_chars, vec!['a']);
}

#[test]
fn test_into_iter_special_chars() {
    let wrapper = StringWrapper("!@#");
    let collected_chars: Vec<char> = wrapper.into_iter().collect();
    assert_eq!(collected_chars, vec!['!', '@', '#']);
}

