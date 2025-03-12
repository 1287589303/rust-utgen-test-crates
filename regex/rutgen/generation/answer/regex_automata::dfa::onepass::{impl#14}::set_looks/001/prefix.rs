// Answer 0

#[test]
fn test_set_looks_with_zero_values() {
    let self_epsilons = Epsilons(0);
    let look_set = LookSet { bits: 0 };
    self_epsilons.set_looks(look_set);
}

#[test]
fn test_set_looks_with_max_look_set() {
    let self_epsilons = Epsilons(0);
    let look_set = LookSet { bits: 1023 };
    self_epsilons.set_looks(look_set);
}

#[test]
fn test_set_looks_with_mid_value_look_set() {
    let self_epsilons = Epsilons(0);
    let look_set = LookSet { bits: 512 };
    self_epsilons.set_looks(look_set);
}

#[test]
fn test_set_looks_with_non_zero_self() {
    let self_epsilons = Epsilons(0x000003FF_FFFFFC00);
    let look_set = LookSet { bits: 0 };
    self_epsilons.set_looks(look_set);
}

#[test]
fn test_set_looks_combined_values() {
    let self_epsilons = Epsilons(0x000003FF_FFFFFC00);
    let look_set = LookSet { bits: 512 };
    self_epsilons.set_looks(look_set);
}

