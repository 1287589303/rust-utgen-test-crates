pub unsafe fn write_mantissa_long(mut output: u64, mut result: *mut u8) {
    if (output >> 32) != 0 {
        // One expensive 64-bit division.
        let mut output2 = (output - 100_000_000 * (output / 100_000_000)) as u32;
        output /= 100_000_000;

        let c = output2 % 10_000;
        output2 /= 10_000;
        let d = output2 % 10_000;
        let c0 = (c % 100) << 1;
        let c1 = (c / 100) << 1;
        let d0 = (d % 100) << 1;
        let d1 = (d / 100) << 1;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c0 as isize),
            result.offset(-2),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c1 as isize),
            result.offset(-4),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(d0 as isize),
            result.offset(-6),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(d1 as isize),
            result.offset(-8),
            2,
        );
        result = result.offset(-8);
    }
    write_mantissa(output as u32, result);
}