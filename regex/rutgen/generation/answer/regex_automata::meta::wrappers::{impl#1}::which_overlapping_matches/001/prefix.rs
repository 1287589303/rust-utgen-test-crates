// Answer 0

#[test]
fn test_which_overlapping_matches_valid_cache_valid_input_empty_haystack() {
    let mut cache = PikeVMCache(Some(pikevm::Cache {}));
    let input = Input {
        haystack: &[],
        span: Span::new(0, 0),
        anchored: Anchored::False,
        earliest: false,
    };
    let mut patset = PatternSet {
        len: 0,
        which: alloc::boxed::Box::new([]),
    };
    let pike_vm = PikeVM(PikeVMEngine(pikevm::PikeVM {}));
    pike_vm.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_valid_cache_valid_input_non_empty_haystack() {
    let mut cache = PikeVMCache(Some(pikevm::Cache {}));
    let input = Input {
        haystack: &[1, 2, 3, 4],
        span: Span::new(0, 4),
        anchored: Anchored::True,
        earliest: true,
    };
    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true]),
    };
    let pike_vm = PikeVM(PikeVMEngine(pikevm::PikeVM {}));
    pike_vm.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_invalid_cache() {
    let mut cache = PikeVMCache(None);
    let input = Input {
        haystack: &[1, 2, 3, 4],
        span: Span::new(0, 4),
        anchored: Anchored::False,
        earliest: false,
    };
    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([false]),
    };
    let pike_vm = PikeVM(PikeVMEngine(pikevm::PikeVM {}));
    pike_vm.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_empty_pattern_set() {
    let mut cache = PikeVMCache(Some(pikevm::Cache {}));
    let input = Input {
        haystack: &[5, 6, 7],
        span: Span::new(0, 3),
        anchored: Anchored::True,
        earliest: true,
    };
    let mut patset = PatternSet {
        len: 0,
        which: alloc::boxed::Box::new([]),
    };
    let pike_vm = PikeVM(PikeVMEngine(pikevm::PikeVM {}));
    pike_vm.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_full_pattern_set() {
    let mut cache = PikeVMCache(Some(pikevm::Cache {}));
    let input = Input {
        haystack: &[8, 9, 10, 11],
        span: Span::new(0, 4),
        anchored: Anchored::False,
        earliest: false,
    };
    let mut patset = PatternSet {
        len: 2,
        which: alloc::boxed::Box::new([true, false]),
    };
    let pike_vm = PikeVM(PikeVMEngine(pikevm::PikeVM {}));
    pike_vm.which_overlapping_matches(&mut cache, &input, &mut patset);
}

