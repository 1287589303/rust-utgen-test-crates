// Answer 0

#[test]
fn test_create_cache_with_prefilter() {
    let regex_info = RegexInfo::new("valid_pattern").unwrap();
    let pre_filter = Some(Prefilter::new());
    let nfa = NFA::new(regex_info.clone()).unwrap();
    let pike_vm = PikeVM::new(&regex_info, pre_filter, &nfa).unwrap();
    let cache = pike_vm.create_cache();
}

#[test]
fn test_create_cache_without_prefilter() {
    let regex_info = RegexInfo::new("another_valid_pattern").unwrap();
    let pre_filter = None;
    let nfa = NFA::new(regex_info.clone()).unwrap();
    let pike_vm = PikeVM::new(&regex_info, pre_filter, &nfa).unwrap();
    let cache = pike_vm.create_cache();
}

