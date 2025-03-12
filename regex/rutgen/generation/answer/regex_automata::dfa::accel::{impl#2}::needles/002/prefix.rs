// Answer 0

#[test]
fn test_needles_valid_index_zero() {
    let accels_data: &[u32] = &[1, 4, 5, 6, 7];
    let accels = Accels { accels: accels_data };
    let result = accels.needles(0);
}

#[test]
fn test_needles_valid_index_one() {
    let accels_data: &[u32] = &[2, 4, 5, 6, 7, 8, 9];
    let accels = Accels { accels: accels_data };
    let result = accels.needles(1);
}

#[test]
fn test_needles_valid_index_boundary() {
    let accels_data: &[u32] = &[3, 2, 3, 4, 5, 6, 7, 8, 9];
    let accels = Accels { accels: accels_data };
    let result = accels.needles(2);
}

#[test]
#[should_panic(expected = "invalid accelerator index 3")]
fn test_needles_invalid_index() {
    let accels_data: &[u32] = &[3, 2, 3, 4, 5, 6, 7, 8, 9];
    let accels = Accels { accels: accels_data };
    let result = accels.needles(3);
}

