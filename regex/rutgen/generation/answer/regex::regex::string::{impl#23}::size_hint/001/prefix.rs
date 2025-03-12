// Answer 0

#[test]
fn test_size_hint_empty_haystack() {
    let haystack: &str = "";
    let it = meta::SplitN::new(haystack);
    let split_n = SplitN { haystack, it };
    split_n.size_hint();
}

#[test]
fn test_size_hint_single_split_character() {
    let haystack: &str = "a,";
    let it = meta::SplitN::new(haystack);
    let split_n = SplitN { haystack, it };
    split_n.size_hint();
}

#[test]
fn test_size_hint_multiple_split_characters() {
    let haystack: &str = "a,b,c,d";
    let it = meta::SplitN::new(haystack);
    let split_n = SplitN { haystack, it };
    split_n.size_hint();
}

#[test]
fn test_size_hint_no_splits() {
    let haystack: &str = "abcd";
    let it = meta::SplitN::new(haystack);
    let split_n = SplitN { haystack, it };
    split_n.size_hint();
}

