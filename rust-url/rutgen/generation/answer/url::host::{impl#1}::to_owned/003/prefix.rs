// Answer 0

#[test]
fn test_to_owned_with_ascii_domain() {
    let host = Host::Domain("example.com");
    let _result = host.to_owned();
}

#[test]
fn test_to_owned_with_non_ascii_domain() {
    let host = Host::Domain("xn--c1yn36f.com"); // Punycode for non-ASCII domain
    let _result = host.to_owned();
}

#[test]
fn test_to_owned_with_empty_domain() {
    let host = Host::Domain("");
    let _result = host.to_owned();
}

#[test]
fn test_to_owned_with_maximum_length_domain() {
    let host = Host::Domain("a".repeat(63) + "." + &"b".repeat(63) + "." + &"c".repeat(63)); // 253 characters
    let _result = host.to_owned();
}

