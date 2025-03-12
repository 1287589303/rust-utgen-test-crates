// Answer 0

#[test]
fn test_cloned_right_variant_with_cloneable_r() {
    struct Cloneable(u32);

    impl Clone for Cloneable {
        fn clone(&self) -> Self {
            Cloneable(self.0)
        }
    }

    let either: Either<&u32, &Cloneable> = Right(&Cloneable(42));
    let result = either.cloned();
}

#[test]
fn test_cloned_right_variant_with_cloneable_string() {
    let either: Either<&u32, &String> = Right(&String::from("hello"));
    let result = either.cloned();
}

#[test]
fn test_cloned_right_variant_empty_string() {
    let either: Either<&u32, &String> = Right(&String::from(""));
    let result = either.cloned();
}

#[test]
fn test_cloned_right_variant_with_zero() {
    let zero = 0u32;
    let either: Either<&u32, &u32> = Right(&zero);
    let result = either.cloned();
}

#[test]
fn test_cloned_right_variant_large_number() {
    let large_number = 1_000_000u32;
    let either: Either<&u32, &u32> = Right(&large_number);
    let result = either.cloned();
}

