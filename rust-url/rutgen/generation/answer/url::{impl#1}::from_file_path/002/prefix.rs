// Answer 0

#[test]
fn test_from_file_path_unix_valid() {
    let url = Url::from_file_path("/tmp/foo.txt").unwrap();
}

#[test]
fn test_from_file_path_unix_valid_home() {
    let url = Url::from_file_path("/home/user/docs/report.pdf").unwrap();
}

#[test]
fn test_from_file_path_windows_valid_disk() {
    let url = Url::from_file_path("C:\\Users\\User\\Documents\\file.txt").unwrap();
}

#[test]
fn test_from_file_path_windows_valid_unc() {
    let url = Url::from_file_path("\\\\Server\\Share\\file.txt").unwrap();
}

#[test]
#[should_panic]
fn test_from_file_path_unix_invalid_relative() {
    let _ = Url::from_file_path("../foo.txt").unwrap();
}

#[test]
#[should_panic]
fn test_from_file_path_unix_invalid_relative_no_prefix() {
    let _ = Url::from_file_path("file.txt").unwrap();
}

#[test]
#[should_panic]
fn test_from_file_path_invalid_url() {
    let _ = Url::from_file_path("https://google.com/").unwrap();
}

#[test]
#[should_panic]
fn test_from_file_path_invalid_path_special_chars() {
    let _ = Url::from_file_path("/path with spaces/foo.txt").unwrap();
}

#[test]
#[should_panic]
fn test_from_file_path_windows_long_path() {
    let long_path = "C:\\".to_owned() + &"a".repeat(260);
    let _ = Url::from_file_path(long_path).unwrap();
}

