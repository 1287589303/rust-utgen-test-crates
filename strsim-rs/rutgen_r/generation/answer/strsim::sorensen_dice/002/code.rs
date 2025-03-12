// Answer 0

#[test]
fn test_sorensen_dice_a_empty_b_not_empty() {
    assert_eq!(0.0, sorensen_dice("", "a"));
}

#[test]
fn test_sorensen_dice_a_one_character_b_not_empty() {
    assert_eq!(0.0, sorensen_dice("a", "b"));
}

#[test]
fn test_sorensen_dice_a_two_characters_b_one_character() {
    assert_eq!(0.0, sorensen_dice("ab", "c"));
}

#[test]
fn test_sorensen_dice_a_empty_b_two_characters() {
    assert_eq!(0.0, sorensen_dice("", "cd"));
}

#[test]
fn test_sorensen_dice_a_one_character_b_two_characters() {
    assert_eq!(0.0, sorensen_dice("a", "cd"));
}

