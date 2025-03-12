// Answer 0

#[test]
fn test_url_origin_blob_valid_parsed() {
    let valid_blob_url = Url::parse("blob:http://example.com/resource").unwrap();
    let origin = url_origin(&valid_blob_url);
}

#[test]
fn test_url_origin_blob_valid_path_parsed() {
    let valid_blob_url_with_path = Url::parse("blob:http://example.com/resource/path").unwrap();
    let origin = url_origin(&valid_blob_url_with_path);
}

#[test]
fn test_url_origin_blob_valid_complex() {
    let complex_blob_url = Url::parse("blob:http://example.com/path/to/resource").unwrap();
    let origin = url_origin(&complex_blob_url);
}

