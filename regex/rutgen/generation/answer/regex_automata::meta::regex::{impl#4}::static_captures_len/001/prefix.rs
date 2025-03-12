// Answer 0

#[test]
fn test_static_captures_len_valid_patterns() {
    let re = Regex::new("a").unwrap();
    re.static_captures_len();
    
    let re = Regex::new("(a)").unwrap();
    re.static_captures_len();
    
    let re = Regex::new("(a)|(b)").unwrap();
    re.static_captures_len();
    
    let re = Regex::new("(a)(b)|(c)(d)").unwrap();
    re.static_captures_len();
    
    let re = Regex::new("(b)+").unwrap();
    re.static_captures_len();
}

#[test]
fn test_static_captures_len_patterns_with_none() {
    let re = Regex::new("(a)|b").unwrap();
    re.static_captures_len();
    
    let re = Regex::new("a|(b)").unwrap();
    re.static_captures_len();
    
    let re = Regex::new("(b)*").unwrap();
    re.static_captures_len();
}

#[test]
fn test_static_captures_len_multiple_patterns() {
    let re = Regex::new_many(&["a", "b"]).unwrap();
    re.static_captures_len();
    
    let re = Regex::new_many(&["(a)", "(b)"]).unwrap();
    re.static_captures_len();
    
    let re = Regex::new_many(&["(a)|(b)", "(c)|(d)"]).unwrap();
    re.static_captures_len();
    
    let re = Regex::new_many(&["(a)(b)|(c)(d)", "(x)(y)"]).unwrap();
    re.static_captures_len();
    
    let re = Regex::new_many(&["(a)", "b"]).unwrap();
    re.static_captures_len();
    
    let re = Regex::new_many(&["a", "(b)"]).unwrap();
    re.static_captures_len();
    
    let re = Regex::new_many(&["(a)", "(b)*"]).unwrap();
    re.static_captures_len();
    
    let re = Regex::new_many(&["(a)+", "(b)+"]).unwrap();
    re.static_captures_len();
}

