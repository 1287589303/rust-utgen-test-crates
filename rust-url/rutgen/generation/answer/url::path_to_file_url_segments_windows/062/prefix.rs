// Answer 0

#[test]
fn test_path_to_file_url_segments_windows_relative_path() {
    let path = Path::new("folder/file.txt");
    let mut serialization = String::new();
    let result = path_to_file_url_segments_windows(&path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_root_relative_path() {
    let path = Path::new("C:/folder/");
    let mut serialization = String::new();
    let result = path_to_file_url_segments_windows(&path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_invalid_path() {
    let path = Path::new("../file.txt");
    let mut serialization = String::new();
    let result = path_to_file_url_segments_windows(&path, &mut serialization);
}

