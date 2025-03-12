// Answer 0

#[test]
fn test_empty_lookset_debug_output() {
    let empty_lookset = LookSet::empty();
    let _ = format!("{:?}", empty_lookset);
}

#[test]
fn test_full_lookset_debug_output() {
    let full_lookset = LookSet::full();
    let _ = format!("{:?}", full_lookset);
}

#[test]
fn test_singleton_lookset_debug_output() {
    let singleton_lookset = LookSet::singleton(Look::Start);
    let _ = format!("{:?}", singleton_lookset);
}

#[test]
fn test_lookset_iter_debug_output() {
    let non_empty_lookset = LookSet::singleton(Look::End);
    let iter = non_empty_lookset.iter();
    let _ = iter.collect::<Vec<_>>();
}

