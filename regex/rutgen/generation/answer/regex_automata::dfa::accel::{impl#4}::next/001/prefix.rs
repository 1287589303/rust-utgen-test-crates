// Answer 0

#[test]
fn test_next_empty_accels() {
    let accels = Accels { accels: &[] };
    let mut iter = IterAccels { accels: &accels, i: 0 };
    let result = iter.next();
}

#[test]
fn test_next_out_of_bounds() {
    let accels = Accels { accels: &[1, 2, 3, 4] };
    let mut iter = IterAccels { accels: &accels, i: 4 };
    let result = iter.next();
}

#[test]
fn test_next_exceeding_length() {
    let accels = Accels { accels: &[5, 6, 7] };
    let mut iter = IterAccels { accels: &accels, i: 10 };
    let result = iter.next();
}

