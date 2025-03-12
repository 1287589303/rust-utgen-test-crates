use crate::lib::*;
use crate::de::{
    Deserialize, Deserializer, EnumAccess, Error, MapAccess, SeqAccess, Unexpected,
    VariantAccess, Visitor,
};
use crate::seed::InPlaceSeed;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::de::size_hint;
pub struct T;
#[cfg(any(feature = "std", feature = "alloc"))]
fn nop_reserve<T>(_seq: T, _n: usize) {}
