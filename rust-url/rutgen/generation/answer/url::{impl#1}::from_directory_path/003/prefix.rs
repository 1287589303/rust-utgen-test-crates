// Answer 0

#[test]
fn test_from_directory_path_unix_no_trailing_slash() {
    let path = std::path::Path::new("/var/www");
    let result = Url::from_directory_path(&path);
    let _ = result.unwrap();
}

#[test]
fn test_from_directory_path_windows_no_trailing_slash() {
    let path = std::path::Path::new("C:\\Users\\Username\\Documents");
    let result = Url::from_directory_path(&path);
    let _ = result.unwrap();
}

#[test]
fn test_from_directory_path_windows_no_trailing_slash_unc() {
    let path = std::path::Path::new("\\\\Server\\Share\\Folder");
    let result = Url::from_directory_path(&path);
    let _ = result.unwrap();
}

