// Answer 0

#[test]
fn test_replace_all_non_empty_haystack_with_simple_replacement() {
    let re = Regex::new(r"\w+").unwrap();
    let replacement = |caps: &Captures| caps[0].to_string();
    let _result = re.replace_all("hello world", replacement);
}

#[test]
fn test_replace_all_empty_haystack_with_simple_replacement() {
    let re = Regex::new(r"\w+").unwrap();
    let replacement = |caps: &Captures| caps[0].to_string();
    let _result = re.replace_all("", replacement);
}

#[test]
fn test_replace_all_haystack_with_only_whitespace() {
    let re = Regex::new(r"\w+").unwrap();
    let replacement = |caps: &Captures| caps[0].to_string();
    let _result = re.replace_all("     ", replacement);
}

#[test]
fn test_replace_all_non_empty_haystack_with_length_check_replacement() {
    let re = Regex::new(r"\w+").unwrap();
    let replacement = |caps: &Captures| {
        if caps[0].len() >= 5 {
            return "long_word".to_string();
        }
        caps[0].to_string()
    };
    let _result = re.replace_all("hi there friends", replacement);
}

#[test]
#[should_panic]
fn test_replace_all_haystack_with_error_returning_replacement() {
    let re = Regex::new(r"\w+").unwrap();
    let replacement = |caps: &Captures| {
        if caps[0].len() >= 5 {
            return Err("word too long");
        }
        Ok(caps[0].to_string())
    };
    let _result = re.replace_all("hi there friends", replacement);
}

#[test]
fn test_replace_all_multi_line_haystack() {
    let re = Regex::new(r"(?m)^\S+").unwrap();
    let replacement = |caps: &Captures| format!("{}!", caps[0]);
    let _result = re.replace_all("hello\nworld\nregex", replacement);
}

