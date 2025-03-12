// Answer 0

#[test]
fn test_path_to_file_url_segments_windows_invalid_server() {
    let mut serialization = String::new();
    let path = std::path::Path::new(r"\\server\share\additional\path");
    let result = path_to_file_url_segments_windows(path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_invalid_server_with_non_ascii() {
    let mut serialization = String::new();
    let path = std::path::Path::new(r"\\server-ä\share\path");
    let result = path_to_file_url_segments_windows(path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_invalid_server_with_invalid_chars() {
    let mut serialization = String::new();
    let path = std::path::Path::new(r"\\server#name\share\path");
    let result = path_to_file_url_segments_windows(path, &mut serialization);
}

