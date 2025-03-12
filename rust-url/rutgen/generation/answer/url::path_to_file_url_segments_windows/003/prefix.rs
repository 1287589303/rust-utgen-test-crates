// Answer 0

#[test]
fn test_path_to_file_url_segments_windows_invalid_prefix() {
    use std::path::{Path, PathBuf};

    let mut serialization = String::new();
    serialization.push_str("initial/");
    let path = PathBuf::from(r"\\invalid\path");

    let result = path_to_file_url_segments_windows(&path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_invalid_prefix2() {
    use std::path::{Path, PathBuf};

    let mut serialization = String::from("start/");
    let path = PathBuf::from(r"\\another\invalid\path");

    let result = path_to_file_url_segments_windows(&path, &mut serialization);
}

