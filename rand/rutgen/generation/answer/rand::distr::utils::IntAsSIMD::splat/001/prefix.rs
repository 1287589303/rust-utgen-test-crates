// Answer 0

#[cfg(test)]
fn test_splat_u32() {
    struct U32Wrapper(u32);
    
    let min_value = U32Wrapper(u32::MIN);
    let zero_value = U32Wrapper(0);
    let positive_value = U32Wrapper(42);
    let max_value = U32Wrapper(u32::MAX);

    U32Wrapper::splat(min_value.0);
    U32Wrapper::splat(zero_value.0);
    U32Wrapper::splat(positive_value.0);
    U32Wrapper::splat(max_value.0);
}

#[cfg(test)]
fn test_splat_i32() {
    struct I32Wrapper(i32);

    let min_value = I32Wrapper(i32::MIN);
    let negative_value = I32Wrapper(-1);
    let zero_value = I32Wrapper(0);
    let positive_value = I32Wrapper(1);
    let max_value = I32Wrapper(i32::MAX);

    I32Wrapper::splat(min_value.0);
    I32Wrapper::splat(negative_value.0);
    I32Wrapper::splat(zero_value.0);
    I32Wrapper::splat(positive_value.0);
    I32Wrapper::splat(max_value.0);
}

#[cfg(test)]
fn test_splat_f32() {
    struct F32Wrapper(f32);

    let min_value = F32Wrapper(f32::MIN);
    let negative_value = F32Wrapper(-1.0);
    let zero_value = F32Wrapper(0.0);
    let positive_value = F32Wrapper(1.0);
    let max_value = F32Wrapper(f32::MAX);

    F32Wrapper::splat(min_value.0);
    F32Wrapper::splat(negative_value.0);
    F32Wrapper::splat(zero_value.0);
    F32Wrapper::splat(positive_value.0);
    F32Wrapper::splat(max_value.0);
}

