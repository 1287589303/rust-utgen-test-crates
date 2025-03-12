// Answer 0

#[test]
fn test_path_to_file_url_segments_windows_verbatim_unc() {
    use std::path::{Path, Component};

    let mut serialization = String::new();
    let path = Path::new(r"\\server\share\folder%20name");
    let result = path_to_file_url_segments_windows(path, &mut serialization);

    // method call
    result.unwrap();
}

#[test]
fn test_path_to_file_url_segments_windows_verbatim_unc_with_empty_share() {
    use std::path::{Path, Component};

    let mut serialization = String::new();
    let path = Path::new(r"\\server\");
    let result = path_to_file_url_segments_windows(path, &mut serialization);

    // method call
    result.unwrap();
}

#[test]
fn test_path_to_file_url_segments_windows_verbatim_unc_non_empty_folder() {
    use std::path::{Path, Component};

    let mut serialization = String::new();
    let path = Path::new(r"\\server\share\folder_with_space");
    let result = path_to_file_url_segments_windows(path, &mut serialization);

    // method call
    result.unwrap();
}

