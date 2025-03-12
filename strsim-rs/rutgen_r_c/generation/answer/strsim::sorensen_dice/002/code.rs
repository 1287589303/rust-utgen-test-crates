// Answer 0

#[test]
fn test_sorensen_dice_empty_string_and_char() {
    let result = sorensen_dice("", "a");
    assert_eq!(0.0, result);
}

#[test]
fn test_sorensen_dice_two_dissimilar_strings() {
    let result = sorensen_dice("french", "quebec");
    assert_eq!(0.0, result);
}

#[test]
fn test_sorensen_dice_one_character_strings() {
    let result = sorensen_dice("a", "b");
    assert_eq!(0.0, result);
} 

#[test]
fn test_sorensen_dice_one_character_and_empty_string() {
    let result = sorensen_dice("a", "");
    assert_eq!(0.0, result);
}

