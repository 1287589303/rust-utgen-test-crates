// Answer 0

#[test]
fn test_is_ascii_empty_label() {
    let label: &[char] = &[];
    is_ascii(label);
}

#[test]
fn test_is_ascii_all_ascii() {
    let label: &[char] = &['a', 'b', 'c', '1', '2', '3', '!', '@', '#'];
    is_ascii(label);
}

#[test]
fn test_is_ascii_max_ascii_length() {
    let label: Vec<char> = ('a'..='z').chain('A'..='Z').chain('0'..='9').chain("!@#$%^&*()_+").collect();
    let label: &[char] = &label;
    is_ascii(label);
}

#[test]
#[should_panic]
fn test_is_ascii_contains_non_ascii() {
    let label: &[char] = &['a', 'b', 'c', '中'];
    is_ascii(label);
}

