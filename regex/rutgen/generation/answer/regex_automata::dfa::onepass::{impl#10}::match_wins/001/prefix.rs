// Answer 0

#[test]
fn test_match_wins_bit_zero() {
    let transition = Transition(0);
    transition.match_wins();
}

#[test]
fn test_match_wins_bit_one() {
    let transition = Transition(1 << Transition::MATCH_WINS_SHIFT);
    transition.match_wins();
}

#[test]
fn test_match_wins_bit_one_minus_one() {
    let transition = Transition((1 << Transition::MATCH_WINS_SHIFT) - 1);
    transition.match_wins();
}

#[test]
fn test_match_wins_bit_one_plus_one() {
    let transition = Transition((1 << Transition::MATCH_WINS_SHIFT) + 1);
    transition.match_wins();
}

#[test]
fn test_match_wins_bit_one_plus_half() {
    let transition = Transition((1 << Transition::MATCH_WINS_SHIFT) + (1 << (Transition::MATCH_WINS_SHIFT - 1)));
    transition.match_wins();
}

