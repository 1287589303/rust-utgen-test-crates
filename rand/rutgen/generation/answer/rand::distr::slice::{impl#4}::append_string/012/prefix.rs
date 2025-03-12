// Answer 0

#[test]
fn test_append_string_boundary_conditions() {
    let slice: [char; 200] = ['a'; 200];
    let rng = &mut rand::thread_rng();
    let mut result_string = String::new();
    let choose = Choose {
        slice: &slice,
        range: UniformUsize { low: 0, range: 200, thresh: 200, mode64: false },
        num_choices: NonZeroUsize::new(1).unwrap(),
    };

    choose.append_string(rng, &mut result_string, 100);
}

