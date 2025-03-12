// Answer 0

#[test]
fn test_next_valid_url_encoded() {
    let input: &[u8] = b"name%3Dvalue&key%40example";
    let parse = Parse { input };
    let mut parse_owned = ParseIntoOwned { inner: parse };
    let _ = parse_owned.next();
}

#[test]
fn test_next_empty_input() {
    let input: &[u8] = b"";
    let parse = Parse { input };
    let mut parse_owned = ParseIntoOwned { inner: parse };
    let _ = parse_owned.next();
}

#[test]
fn test_next_special_characters() {
    let input: &[u8] = b"key%20with%20spaces&value%21%40%23%24%25";
    let parse = Parse { input };
    let mut parse_owned = ParseIntoOwned { inner: parse };
    let _ = parse_owned.next();
}

#[test]
fn test_next_reserved_characters() {
    let input: &[u8] = b"key%2Fwith%2Fslashes&value%3Awith%3Acolons";
    let parse = Parse { input };
    let mut parse_owned = ParseIntoOwned { inner: parse };
    let _ = parse_owned.next();
}

#[test]
fn test_next_multiple_pairs() {
    let input: &[u8] = b"first%3D1&second%3D2&third%3D3";
    let parse = Parse { input };
    let mut parse_owned = ParseIntoOwned { inner: parse };
    let _ = parse_owned.next();
    let _ = parse_owned.next();
    let _ = parse_owned.next();
}

