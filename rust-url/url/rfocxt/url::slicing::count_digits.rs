use core::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};
use crate::Url;
fn count_digits(n: u16) -> usize {
    match n {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        10000..=65535 => 5,
    }
}
