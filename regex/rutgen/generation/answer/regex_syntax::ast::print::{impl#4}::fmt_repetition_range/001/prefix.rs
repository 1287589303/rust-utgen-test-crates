// Answer 0

#[test]
fn test_fmt_repetition_range_bounded_zero() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    let ast = ast::RepetitionRange::Bounded(0, 0);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded_zero_to_ten() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    let ast = ast::RepetitionRange::Bounded(0, 10);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded_one_to_one() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    let ast = ast::RepetitionRange::Bounded(1, 1);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded_two_to_three() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    let ast = ast::RepetitionRange::Bounded(2, 3);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded_ten_to_ten() {
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    let ast = ast::RepetitionRange::Bounded(10, 10);
    writer.fmt_repetition_range(&ast).unwrap();
}

