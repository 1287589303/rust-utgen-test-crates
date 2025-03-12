// Answer 0

#[test]
fn test_next_at_end_byte() {
    let mut byte_classes = ByteClasses::empty();
    let representatives = ByteClassRepresentatives {
        classes: &byte_classes,
        cur_byte: 256,
        end_byte: Some(256),
        last_class: None,
    };
    let result = representatives.next();
}

#[test]
fn test_next_at_end_byte_not_max() {
    let mut byte_classes = ByteClasses::singletons();
    let mut representatives = ByteClassRepresentatives {
        classes: &byte_classes,
        cur_byte: 256,
        end_byte: Some(256),
        last_class: None,
    };
    let result = representatives.next();
}

