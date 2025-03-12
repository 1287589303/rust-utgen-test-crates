// Answer 0

#[test]
fn test_source_duplicate_error() {
    use crate::util::primitives::PatternID;

    let pattern_id = PatternID::new(1).unwrap(); // Assuming PatternID::new takes a usize and returns a PatternID
    let duplicate_name = String::from("duplicate_name");

    let error = GroupInfoError {
        kind: GroupInfoErrorKind::Duplicate {
            pattern: pattern_id,
            name: duplicate_name,
        },
    };

    let _result = error.source(); // Invoking the method to be tested
}

