// Answer 0

#[test]
fn test_sorensen_dice_distinct_bigrams() {
    let a = "ab";
    let b = "cd";
    let _ = sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_another_distinct_bigrams() {
    let a = "xy";
    let b = "zw";
    let _ = sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_different_chars() {
    let a = "pq";
    let b = "rs";
    let _ = sorensen_dice(a, b);
}

