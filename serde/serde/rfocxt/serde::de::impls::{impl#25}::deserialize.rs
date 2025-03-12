use crate::lib::*;
use crate::de::{
    Deserialize, Deserializer, EnumAccess, Error, MapAccess, SeqAccess, Unexpected,
    VariantAccess, Visitor,
};
use crate::seed::InPlaceSeed;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::de::size_hint;
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        note = "for local types consider adding `#[derive(serde::Deserialize)]` to your `{Self}` type",
        note = "for types from other crates check whether the crate offers a `serde` feature flag",
    )
)]
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        *place = tri!(Deserialize::deserialize(deserializer));
        Ok(())
    }
}
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        note = "for local types consider adding `#[derive(serde::Serialize)]` to your `{Self}` type",
        note = "for types from other crates check whether the crate offers a `serde` feature flag",
    )
)]
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
#[cfg(any(feature = "std", not(no_core_net)))]
struct FromStrVisitor<T> {
    expecting: &'static str,
    ty: PhantomData<T>,
}
#[cfg(any(feature = "std", not(no_core_net)))]
impl<'de> Deserialize<'de> for net::IpAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(FromStrVisitor::new("IP address"))
        } else {
            use crate::lib::net::IpAddr;
            deserialize_enum! {
                IpAddr IpAddrKind(V4; b"V4"; 0, V6; b"V6"; 1) "`V4` or `V6`",
                deserializer
            }
        }
    }
}
#[cfg(any(feature = "std", not(no_core_net)))]
impl<T> FromStrVisitor<T> {
    fn new(expecting: &'static str) -> Self {
        FromStrVisitor {
            expecting,
            ty: PhantomData,
        }
    }
}
