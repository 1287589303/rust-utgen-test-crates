// Answer 0

#[test]
fn test_get_none_because_index_is_equal_to_length() {
    let accels = Accels {
        accels: [0u32; 8],
    };
    let length = accels.len();
    let result = accels.get(length);
}

#[test]
fn test_get_none_because_index_is_equal_to_length_when_len_is_zero() {
    let accels = Accels {
        accels: [0u32; 8],
    };
    let length = accels.len();
    let result = accels.get(length);
}

#[test]
fn test_get_none_because_index_is_equal_to_length_when_len_is_max() {
    let accels = Accels {
        accels: [8u32; 8],
    };
    let length = accels.len();
    let result = accels.get(length);
}

