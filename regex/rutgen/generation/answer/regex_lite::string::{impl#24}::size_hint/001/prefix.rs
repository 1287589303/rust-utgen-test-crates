// Answer 0

#[test]
fn test_size_hint_non_empty_haystack() {
    let haystack = "abc|def|ghi";
    let finder = Matches::new(haystack, "|");
    let splits = Split {
        haystack,
        finder,
        last: 0,
    };
    let split_n = SplitN { splits, limit: 2 };
    let hint = split_n.size_hint();
}

#[test]
fn test_size_hint_empty_haystack() {
    let haystack = "";
    let finder = Matches::new(haystack, "|");
    let splits = Split {
        haystack,
        finder,
        last: 0,
    };
    let split_n = SplitN { splits, limit: 2 };
    let hint = split_n.size_hint();
}

#[test]
fn test_size_hint_limit_zero() {
    let haystack = "abc|def|ghi";
    let finder = Matches::new(haystack, "|");
    let splits = Split {
        haystack,
        finder,
        last: 0,
    };
    let split_n = SplitN { splits, limit: 0 };
    let hint = split_n.size_hint();
}

#[test]
fn test_size_hint_limit_one() {
    let haystack = "abc|def|ghi";
    let finder = Matches::new(haystack, "|");
    let splits = Split {
        haystack,
        finder,
        last: 0,
    };
    let split_n = SplitN { splits, limit: 1 };
    let hint = split_n.size_hint();
}

#[test]
fn test_size_hint_limit_max() {
    let haystack = "abc|def|ghi";
    let finder = Matches::new(haystack, "|");
    let splits = Split {
        haystack,
        finder,
        last: 0,
    };
    let split_n = SplitN {
        splits,
        limit: std::usize::MAX,
    };
    let hint = split_n.size_hint();
}

