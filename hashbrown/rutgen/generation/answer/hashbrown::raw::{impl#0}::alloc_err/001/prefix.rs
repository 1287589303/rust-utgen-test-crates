// Answer 0

#[test]
#[should_panic]
fn test_alloc_err_infallible_zero_size_align() {
    let fallibility = Fallibility::Infallible;
    let layout = Layout::from_size_align(0, 1).unwrap();
    let _result = fallibility.alloc_err(layout);
}

#[test]
#[should_panic]
fn test_alloc_err_infallible_max_size() {
    let fallibility = Fallibility::Infallible;
    let layout = Layout::from_size_align(isize::MAX, std::mem::align_of::<usize>()).unwrap();
    let _result = fallibility.alloc_err(layout);
}

#[test]
#[should_panic]
fn test_alloc_err_infallible_small_size() {
    let fallibility = Fallibility::Infallible;
    let layout = Layout::from_size_align(1, std::mem::align_of::<usize>()).unwrap();
    let _result = fallibility.alloc_err(layout);
}

#[test]
#[should_panic]
fn test_alloc_err_infallible_medium_size() {
    let fallibility = Fallibility::Infallible;
    let layout = Layout::from_size_align(1024, std::mem::align_of::<usize>()).unwrap();
    let _result = fallibility.alloc_err(layout);
}

