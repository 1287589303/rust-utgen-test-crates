// Answer 0

#[test]
fn test_multiple_of_power_of_5_32_true() {
    fn pow5factor_32(value: u32) -> u32 {
        let mut count = 0;
        let mut v = value;
        while v % 5 == 0 {
            v /= 5;
            count += 1;
        }
        count
    }

    let value = 25; // 25 = 5^2, pow5factor_32(25) should return 2
    let p = 2;
    assert_eq!(multiple_of_power_of_5_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_32_false() {
    fn pow5factor_32(value: u32) -> u32 {
        let mut count = 0;
        let mut v = value;
        while v % 5 == 0 {
            v /= 5;
            count += 1;
        }
        count
    }

    let value = 10; // 10 = 5^1, pow5factor_32(10) should return 1
    let p = 2;
    assert_eq!(multiple_of_power_of_5_32(value, p), false);
}

#[test]
fn test_multiple_of_power_of_5_32_boundary() {
    fn pow5factor_32(value: u32) -> u32 {
        let mut count = 0;
        let mut v = value;
        while v % 5 == 0 {
            v /= 5;
            count += 1;
        }
        count
    }

    let value = 1; // 1 has no factors of 5, pow5factor_32(1) should return 0
    let p = 0;
    assert_eq!(multiple_of_power_of_5_32(value, p), true);
}

