// Answer 0

#[test]
fn test_utf8_sequence_four_case1() {
    let r = [
        Utf8Range { start: 0, end: 0 },
        Utf8Range { start: 1, end: 1 },
        Utf8Range { start: 2, end: 2 },
        Utf8Range { start: 3, end: 3 },
    ];
    let seq = Utf8Sequence::Four(r);
    let _ = fmt::format(fmt::Formatter::new(), &seq);
}

#[test]
fn test_utf8_sequence_four_case2() {
    let r = [
        Utf8Range { start: 10, end: 20 },
        Utf8Range { start: 21, end: 30 },
        Utf8Range { start: 31, end: 40 },
        Utf8Range { start: 41, end: 50 },
    ];
    let seq = Utf8Sequence::Four(r);
    let _ = fmt::format(fmt::Formatter::new(), &seq);
}

#[test]
fn test_utf8_sequence_four_case3() {
    let r = [
        Utf8Range { start: 100, end: 255 },
        Utf8Range { start: 192, end: 200 },
        Utf8Range { start: 201, end: 210 },
        Utf8Range { start: 211, end: 220 },
    ];
    let seq = Utf8Sequence::Four(r);
    let _ = fmt::format(fmt::Formatter::new(), &seq);
}

#[test]
fn test_utf8_sequence_four_case4() {
    let r = [
        Utf8Range { start: 0, end: 255 },
        Utf8Range { start: 255, end: 255 },
        Utf8Range { start: 255, end: 255 },
        Utf8Range { start: 255, end: 255 },
    ];
    let seq = Utf8Sequence::Four(r);
    let _ = fmt::format(fmt::Formatter::new(), &seq);
}

