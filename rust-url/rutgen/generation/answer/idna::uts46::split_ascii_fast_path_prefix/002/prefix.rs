// Answer 0

#[test]
fn test_split_ascii_fast_path_prefix_valid_case_1() {
    let label: &[u8] = b"abc\xFF"; // ASCII followed by non-ASCII, pos will be 3
    split_ascii_fast_path_prefix(label);
}

#[test]
fn test_split_ascii_fast_path_prefix_valid_case_2() {
    let label: &[u8] = b"xyz\xC2\xA9"; // ASCII followed by non-ASCII (©), pos will be 3
    split_ascii_fast_path_prefix(label);
}

#[test]
fn test_split_ascii_fast_path_prefix_valid_case_3() {
    let label: &[u8] = b"test1234\xE2\x9C\x94"; // ASCII followed by non-ASCII (✔), pos will be 10
    split_ascii_fast_path_prefix(label);
}

#[test]
fn test_split_ascii_fast_path_prefix_valid_case_4() {
    let label: &[u8] = b"a\xE2\x80\x98a"; // ASCII character followed by non-ASCII (‘), pos will be 1
    split_ascii_fast_path_prefix(label);
}

#[test]
fn test_split_ascii_fast_path_prefix_valid_case_5() {
    let label: &[u8] = b"string\xE2\x9A\xA1"; // ASCII followed by non-ASCII (⚡), pos will be 6
    split_ascii_fast_path_prefix(label);
}

