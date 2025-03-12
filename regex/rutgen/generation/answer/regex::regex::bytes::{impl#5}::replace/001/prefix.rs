// Answer 0

#[test]
fn test_replace_with_simple_string() {
    let re = Regex::new(r"[^01]+").unwrap();
    let result = re.replace(b"1078910", b"");
}

#[test]
fn test_replace_with_function() {
    let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();
    let result = re.replace(b"Springsteen, Bruce", |caps: &Captures| {
        let mut buf = vec![];
        buf.extend_from_slice(&caps[2]);
        buf.push(b' ');
        buf.extend_from_slice(&caps[1]);
        buf
    });
}

#[test]
fn test_replace_with_named_capture() {
    let re = Regex::new(r"(?<last>[^,\s]+),\s+(?<first>\S+)").unwrap();
    let result = re.replace(b"Springsteen, Bruce", b"$first $last");
}

#[test]
fn test_replace_with_curly_braces() {
    let re = Regex::new(r"(?<first>\w+)\s+(?<second>\w+)").unwrap();
    let result = re.replace(b"deep fried", b"${first}_$second");
}

#[test]
fn test_replace_with_no_expand() {
    let re = Regex::new(r"(?<last>[^,\s]+),\s+(\S+)").unwrap();
    let result = re.replace(b"Springsteen, Bruce", NoExpand(b"$2 $last"));
}

#[test]
fn test_replace_with_empty_haystack() {
    let re = Regex::new(r"[^01]+").unwrap();
    let result = re.replace(b"", b"replacement");
}

#[test]
fn test_replace_with_single_character() {
    let re = Regex::new(r"[A-Z]").unwrap();
    let result = re.replace(b"A", b"alpha");
}

#[test]
fn test_replace_with_max_length() {
    let re = Regex::new(r"\d+").unwrap();
    let result = re.replace(b"1234567890", b"longreplacement");
}

