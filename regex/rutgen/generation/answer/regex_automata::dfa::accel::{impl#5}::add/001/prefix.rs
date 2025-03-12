// Answer 0

#[test]
fn test_add_when_full() {
    let mut accel = {
        let mut a = Accel::new();
        a.bytes[0] = 3; // Set length to 3
        a.bytes[1] = 1; // Existing byte
        a.bytes[2] = 2; // Existing byte
        a.bytes[3] = 3; // Existing byte
        a
    };
    let result = accel.add(4); // Attempt to add a new byte
}

#[test]
fn test_add_when_space() {
    let mut accel = {
        let mut a = Accel::new();
        a.bytes[0] = 3; // Set length to 3
        a.bytes[1] = 1; // Existing byte
        a.bytes[2] = 2; // Existing byte
        a.bytes[3] = 3; // Existing byte
        a
    };
    let result = accel.add(b' '); // Attempt to add space byte
}

#[test]
fn test_add_when_byte_already_present() {
    let mut accel = {
        let mut a = Accel::new();
        a.bytes[0] = 3; // Set length to 3
        a.bytes[1] = 1; // Existing byte
        a.bytes[2] = 2; // Existing byte
        a.bytes[3] = 3; // Existing byte
        a
    };
    let result = accel.add(1); // Attempt to add an already existing byte
}

