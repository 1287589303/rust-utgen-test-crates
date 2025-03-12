// Answer 0

#[test]
fn test_len_equal_start_at_0() {
    let progress = SearchProgress { start: 0, at: 0 };
    let _ = progress.len();
}

#[test]
fn test_len_equal_start_at_5() {
    let progress = SearchProgress { start: 5, at: 5 };
    let _ = progress.len();
}

#[test]
fn test_len_start_at_0() {
    let progress = SearchProgress { start: 0, at: 10 };
    let _ = progress.len();
}

#[test]
fn test_len_start_at_7() {
    let progress = SearchProgress { start: 5, at: 10 };
    let _ = progress.len();
}

#[test]
fn test_len_start_at_100() {
    let progress = SearchProgress { start: 100, at: 150 };
    let _ = progress.len();
}

