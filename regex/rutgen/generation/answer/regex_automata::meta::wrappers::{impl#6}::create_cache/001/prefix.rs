// Answer 0

#[test]
fn create_cache_with_dfa_onepass_enabled() {
    struct MockRegexInfo;
    struct MockNFA;

    let info = MockRegexInfo;
    let nfa = MockNFA;
    let one_pass = OnePass::new(&info, &nfa);

    let cache = one_pass.create_cache();
}

#[test]
fn create_cache_with_dfa_onepass_disabled() {
    struct MockRegexInfo;
    struct MockNFA;

    let info = MockRegexInfo;
    let nfa = MockNFA;
    let one_pass = OnePass::new(&info, &nfa);

    let cache = one_pass.create_cache();
}

