// Answer 0

#[test]
fn test_contains_start() {
    let lookset = LookSet { bits: 1 };
    let result = lookset.contains(Look::Start);
}

#[test]
fn test_contains_end() {
    let lookset = LookSet { bits: 2 };
    let result = lookset.contains(Look::End);
}

#[test]
fn test_contains_startlf() {
    let lookset = LookSet { bits: 4 };
    let result = lookset.contains(Look::StartLF);
}

#[test]
fn test_contains_endlf() {
    let lookset = LookSet { bits: 8 };
    let result = lookset.contains(Look::EndLF);
}

#[test]
fn test_contains_wordascii() {
    let lookset = LookSet { bits: 64 };
    let result = lookset.contains(Look::WordAscii);
}

#[test]
fn test_contains_wordunicodenegate() {
    let lookset = LookSet { bits: 512 };
    let result = lookset.contains(Look::WordUnicodeNegate);
}

#[test]
fn test_contains_combined() {
    let lookset = LookSet { bits: 3 }; // 1 | 2
    let result_start = lookset.contains(Look::Start);
    let result_end = lookset.contains(Look::End);
}

#[test]
fn test_contains_none() {
    let lookset = LookSet { bits: 0 }; 
    let result = lookset.contains(Look::Start);
}

