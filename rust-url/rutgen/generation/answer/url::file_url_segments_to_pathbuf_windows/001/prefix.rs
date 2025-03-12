// Answer 0

#[test]
fn test_file_url_segments_to_pathbuf_windows_capacity_zero_empty_segments() {
    let estimated_capacity: usize = 0;
    let host: Option<&str> = None;
    let segments = "".split(';');

    let _ = file_url_segments_to_pathbuf_windows(estimated_capacity, host, segments);
}

#[test]
fn test_file_url_segments_to_pathbuf_windows_capacity_zero_single_empty_segment() {
    let estimated_capacity: usize = 0;
    let host: Option<&str> = None;
    let segments = [""];

    let _ = file_url_segments_to_pathbuf_windows(estimated_capacity, host, segments.iter().copied().flat_map(|s| s.split(';')));
}

#[test]
fn test_file_url_segments_to_pathbuf_windows_capacity_zero_multiple_empty_segments() {
    let estimated_capacity: usize = 0;
    let host: Option<&str> = None;
    let segments = ["", "", ""];

    let _ = file_url_segments_to_pathbuf_windows(estimated_capacity, host, segments.iter().copied().flat_map(|s| s.split(';')));
}

