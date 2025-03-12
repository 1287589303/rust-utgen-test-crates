// Answer 0

#[test]
fn test_take_saved_with_to_save_variant() {
    let id = LazyStateID(1234);
    let mut saver = StateSaver::ToSave { id, state: State::default() };
    let result = saver.take_saved();
}

#[test]
fn test_take_saved_with_saved_variant() {
    let id = LazyStateID(5678);
    let mut saver = StateSaver::Saved(id);
    let result = saver.take_saved();
}

#[test]
fn test_take_saved_with_none_variant() {
    let mut saver = StateSaver::None;
    let result = saver.take_saved();
}

