// Answer 0

#[test]
fn test_find_byte_empty_haystack() {
    let needle: u8 = 5;
    let haystack: &[u8] = &[];
    find_byte(needle, haystack);
}

#[test]
fn test_find_byte_not_found() {
    let needle: u8 = 5;
    let haystack: &[u8] = &[1, 2, 3, 4];
    find_byte(needle, haystack);
}

#[test]
fn test_find_byte_found_first() {
    let needle: u8 = 5;
    let haystack: &[u8] = &[5, 1, 2, 3, 4];
    find_byte(needle, haystack);
}

#[test]
fn test_find_byte_found_middle() {
    let needle: u8 = 5;
    let haystack: &[u8] = &[1, 2, 5, 3, 4];
    find_byte(needle, haystack);
}

#[test]
fn test_find_byte_found_last() {
    let needle: u8 = 5;
    let haystack: &[u8] = &[1, 2, 3, 4, 5];
    find_byte(needle, haystack);
}

#[test]
fn test_find_byte_found_multiple() {
    let needle: u8 = 5;
    let haystack: &[u8] = &[1, 5, 3, 5, 4];
    find_byte(needle, haystack);
}

#[test]
fn test_find_byte_all_identical_bytes_found() {
    let needle: u8 = 5;
    let haystack: &[u8] = &[5, 5, 5, 5, 5];
    find_byte(needle, haystack);
}

#[test]
fn test_find_byte_all_identical_bytes_not_found() {
    let needle: u8 = 3;
    let haystack: &[u8] = &[5, 5, 5, 5, 5];
    find_byte(needle, haystack);
}

#[test]
fn test_find_byte_large_haystack_found() {
    let needle: u8 = 5;
    let haystack: Vec<u8> = (0..1000).map(|x| x as u8).collect();
    find_byte(needle, &haystack);
}

#[test]
fn test_find_byte_large_haystack_not_found() {
    let needle: u8 = 255;
    let haystack: Vec<u8> = (0..1000).map(|x| x as u8).collect();
    find_byte(needle, &haystack);
}

