// Answer 0

#[test]
fn test_path_to_file_url_segments_windows_unc() {
    use std::path::{Path, PathBuf};

    let path = Path::new(r"\\servername\share\folder\subfolder");
    let mut serialization = String::new();

    let result = path_to_file_url_segments_windows(path, &mut serialization);

    // The result should be an Ok with populated host_end and host_internal
    // Since there are no assertions, we just check function execution.
    let _ = result;
}

#[test]
fn test_path_to_file_url_segments_windows_unc_with_encoded_chars() {
    use std::path::{Path, PathBuf};

    let path = Path::new(r"\\server-name\my share\folder");
    let mut serialization = String::new();

    let result = path_to_file_url_segments_windows(path, &mut serialization);

    let _ = result;
}

#[test]
fn test_path_to_file_url_segments_windows_unc_with_multiple_components() {
    use std::path::{Path, PathBuf};

    let path = Path::new(r"\\server_name\documents\folder\another_folder");
    let mut serialization = String::new();

    let result = path_to_file_url_segments_windows(path, &mut serialization);

    let _ = result;
}

