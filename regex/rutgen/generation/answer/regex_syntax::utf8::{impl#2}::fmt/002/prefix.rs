// Answer 0

#[test]
fn test_utf8_sequence_three_case_1() {
    let r = [
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
    ];
    let seq = Utf8Sequence::Three(r);
    let _ = format!("{:?}", seq);
}

#[test]
fn test_utf8_sequence_three_case_2() {
    let r = [
        Utf8Range { start: 10, end: 20 },
        Utf8Range { start: 30, end: 40 },
        Utf8Range { start: 50, end: 60 },
    ];
    let seq = Utf8Sequence::Three(r);
    let _ = format!("{:?}", seq);
}

#[test]
fn test_utf8_sequence_three_case_3() {
    let r = [
        Utf8Range { start: 200, end: 210 },
        Utf8Range { start: 220, end: 225 },
        Utf8Range { start: 230, end: 240 },
    ];
    let seq = Utf8Sequence::Three(r);
    let _ = format!("{:?}", seq);
}

#[test]
fn test_utf8_sequence_three_case_4() {
    let r = [
        Utf8Range { start: 255, end: 255 },
        Utf8Range { start: 255, end: 255 },
        Utf8Range { start: 255, end: 255 },
    ];
    let seq = Utf8Sequence::Three(r);
    let _ = format!("{:?}", seq);
}

#[test]
fn test_utf8_sequence_three_edge_case() {
    let r = [
        Utf8Range { start: 0, end: 255 },
        Utf8Range { start: 255, end: 255 },
        Utf8Range { start: 0, end: 0 },
    ];
    let seq = Utf8Sequence::Three(r);
    let _ = format!("{:?}", seq);
}

