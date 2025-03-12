// Answer 0

#[test]
fn test_pattern_len_basic() {
    use crate::hybrid::regex::Regex;

    let re = Regex::new_many(&["[a-z]+", "[0-9]+"]).unwrap();
    let length = re.pattern_len();
}

#[test]
fn test_pattern_len_with_alphabet() {
    use crate::hybrid::regex::Regex;

    let re = Regex::new_many(&["[a-z]+", "[a-zA-Z]+"]).unwrap();
    let length = re.pattern_len();
}

#[test]
fn test_pattern_len_empty() {
    use crate::hybrid::regex::Regex;

    let re = Regex::new_many(&[""]).unwrap();
    let length = re.pattern_len();
}

#[test]
fn test_pattern_len_whitespace_classes() {
    use crate::hybrid::regex::Regex;

    let re = Regex::new_many(&["\\s+", "\\W+"]).unwrap();
    let length = re.pattern_len();
}

#[test]
fn test_pattern_len_multiple_patterns() {
    use crate::hybrid::regex::Regex;

    let re = Regex::new_many(&["[a-z]+", "[0-9]+", "[a-zA-Z]+"]).unwrap();
    let length = re.pattern_len();
}

