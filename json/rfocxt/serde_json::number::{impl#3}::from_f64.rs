#[cfg(feature = "arbitrary_precision")]
type N = String;
use crate::de::ParserNumber;
use crate::error::Error;
#[cfg(feature = "arbitrary_precision")]
use crate::error::ErrorCode;
#[cfg(feature = "arbitrary_precision")]
use alloc::borrow::ToOwned;
#[cfg(feature = "arbitrary_precision")]
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};
#[cfg(not(feature = "arbitrary_precision"))]
use core::hash::{Hash, Hasher};
use serde::de::{self, Unexpected, Visitor};
#[cfg(feature = "arbitrary_precision")]
use serde::de::{IntoDeserializer, MapAccess};
use serde::{
    forward_to_deserialize_any, Deserialize, Deserializer, Serialize, Serializer,
};
#[cfg(feature = "arbitrary_precision")]
pub(crate) const TOKEN: &str = "$serde_json::private::Number";
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}
#[cfg(not(feature = "arbitrary_precision"))]
#[derive(Copy, Clone)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}
impl Number {
    pub fn is_i64(&self) -> bool {}
    pub fn is_u64(&self) -> bool {}
    pub fn is_f64(&self) -> bool {}
    pub fn as_i64(&self) -> Option<i64> {}
    pub fn as_u64(&self) -> Option<u64> {}
    pub fn as_f64(&self) -> Option<f64> {}
    pub fn from_f64(f: f64) -> Option<Number> {
        if f.is_finite() {
            let n = {
                #[cfg(not(feature = "arbitrary_precision"))] { N::Float(f) }
                #[cfg(feature = "arbitrary_precision")]
                { ryu::Buffer::new().format_finite(f).to_owned() }
            };
            Some(Number { n })
        } else {
            None
        }
    }
    pub fn as_i128(&self) -> Option<i128> {}
    pub fn as_u128(&self) -> Option<u128> {}
    pub fn from_i128(i: i128) -> Option<Number> {}
    pub fn from_u128(i: u128) -> Option<Number> {}
    #[cfg(feature = "arbitrary_precision")]
    #[cfg_attr(docsrs, doc(cfg(feature = "arbitrary_precision")))]
    pub fn as_str(&self) -> &str {}
    pub(crate) fn as_f32(&self) -> Option<f32> {}
    pub(crate) fn from_f32(f: f32) -> Option<Number> {}
    #[cfg(feature = "arbitrary_precision")]
    #[inline]
    pub fn from_string_unchecked(n: String) -> Self {
        Number { n }
    }
}
