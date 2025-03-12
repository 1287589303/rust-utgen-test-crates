// Answer 0

#[test]
fn test_pattern_len_never_match() {
    let re = PikeVM::never_match().unwrap();
    let _ = re.pattern_len();
}

#[test]
fn test_pattern_len_always_match() {
    let re = PikeVM::always_match().unwrap();
    let _ = re.pattern_len();
}

#[test]
fn test_pattern_len_new_many() {
    let re = PikeVM::new_many(&["[0-9]+", "[a-z]+", "[A-Z]+"]).unwrap();
    let _ = re.pattern_len();
}

