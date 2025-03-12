// Answer 0

#[test]
fn test_path_to_file_url_segments_windows_valid_disk() {
    use std::path::Path;

    let mut serialization = String::new();
    let path = Path::new("C:/folder/file.txt");

    let result = path_to_file_url_segments_windows(&path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_valid_disk_multiple_components() {
    use std::path::Path;

    let mut serialization = String::new();
    let path = Path::new("D:/another_folder/subfolder/file.dat");

    let result = path_to_file_url_segments_windows(&path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_valid_drive_letter() {
    use std::path::Path;

    let mut serialization = String::new();
    let path = Path::new("E:/projects/example");

    let result = path_to_file_url_segments_windows(&path, &mut serialization);
}

