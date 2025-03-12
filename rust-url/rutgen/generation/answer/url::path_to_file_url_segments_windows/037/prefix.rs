// Answer 0

#[test]
fn test_path_to_file_url_segments_windows_verbatim_disk_non_ascii() {
    use std::path::{Path, Component};
    let mut serialization = String::new();
    let path = Path::new(r"\\.\C:\path\to\resürcé");
    let result = path_to_file_url_segments_windows(path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_verbatim_disk_component_not_root() {
    use std::path::{Path, Component};
    let mut serialization = String::new();
    let path = Path::new(r"\\.\C:\another_directory\resource");
    let result = path_to_file_url_segments_windows(path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_verbatim_disk_root_component_non_last() {
    use std::path::{Path, Component};
    let mut serialization = String::new();
    let path = Path::new(r"\\.\C:\root\");
    let result = path_to_file_url_segments_windows(path, &mut serialization);
}

#[test]
fn test_path_to_file_url_segments_windows_verbatim_disk_empty_last_component() {
    use std::path::{Path, Component};
    let mut serialization = String::new();
    let path = Path::new(r"\\.\C:\resource\");
    let result = path_to_file_url_segments_windows(path, &mut serialization);
}

