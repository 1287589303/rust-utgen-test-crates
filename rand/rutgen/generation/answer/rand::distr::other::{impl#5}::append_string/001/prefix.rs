// Answer 0

#[test]
fn test_append_string_length_zero() {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    let len = 0;
    let alphabetic = Alphabetic;

    alphabetic.append_string(&mut rng, &mut string, len);
}

#[test]
fn test_append_string_length_small() {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    let len = 5;
    let alphabetic = Alphabetic;

    alphabetic.append_string(&mut rng, &mut string, len);
}

#[test]
fn test_append_string_length_mid() {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    let len = 50;
    let alphabetic = Alphabetic;

    alphabetic.append_string(&mut rng, &mut string, len);
}

#[test]
fn test_append_string_length_large() {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    let len = 1000;
    let alphabetic = Alphabetic;

    alphabetic.append_string(&mut rng, &mut string, len);
}

