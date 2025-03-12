// Answer 0

#[test]
#[should_panic]
fn test_serialize_element_invalid_value() {
    struct NonSerializable;

    let mut state = State::First;
    let mut writer: Vec<u8> = Vec::new();
    let formatter = CompactFormatter; // Assuming CompactFormatter is implementable
    let mut serializer = Serializer { writer, formatter };
    
    let mut compound = Compound::Map {
        ser: &mut serializer,
        state: state,
    };
    
    compound.serialize_element(&NonSerializable);
} 

#[test]
#[should_panic]
fn test_serialize_element_with_state_rest() {
    struct NonSerializable;

    let mut state = State::Rest;
    let mut writer: Vec<u8> = Vec::new();
    let formatter = CompactFormatter; // Assuming CompactFormatter is implementable
    let mut serializer = Serializer { writer, formatter };

    let mut compound = Compound::Map {
        ser: &mut serializer,
        state: state,
    };

    compound.serialize_element(&NonSerializable);
} 

