// Answer 0

#[test]
fn test_sorensen_dice_a_len_2_b_len_1() {
    let a = "ab"; 
    let b = "c"; 
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_a_len_2_b_empty() {
    let a = "xy"; 
    let b = ""; 
    sorensen_dice(a, b);
}

