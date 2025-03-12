// Answer 0

#[test]
fn test_next_with_cur_byte_equal_to_end_byte() {
    let classes = ByteClasses([0; 256]);
    let mut representatives = ByteClassRepresentatives {
        classes: &classes,
        cur_byte: 256,
        end_byte: None,
        last_class: None,
    };
    representatives.next();
}

#[test]
fn test_next_with_cur_byte_equal_to_end_byte_and_not_max() {
    let classes = ByteClasses([0; 256]);
    let mut representatives = ByteClassRepresentatives {
        classes: &classes,
        cur_byte: 256,
        end_byte: None,
        last_class: Some(0),
    };
    representatives.next();
}

