#[cfg(not(feature = "preserve_order"))]
type MapImpl<K, V> = BTreeMap<K, V>;
#[cfg(feature = "preserve_order")]
type MapImpl<K, V> = IndexMap<K, V>;
#[cfg(not(feature = "preserve_order"))]
type VacantEntryImpl<'a> = btree_map::VacantEntry<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type VacantEntryImpl<'a> = indexmap::map::VacantEntry<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type OccupiedEntryImpl<'a> = btree_map::OccupiedEntry<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type OccupiedEntryImpl<'a> = indexmap::map::OccupiedEntry<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type IterImpl<'a> = btree_map::Iter<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type IterImpl<'a> = indexmap::map::Iter<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type IterMutImpl<'a> = btree_map::IterMut<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type IterMutImpl<'a> = indexmap::map::IterMut<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type IntoIterImpl = btree_map::IntoIter<String, Value>;
#[cfg(feature = "preserve_order")]
type IntoIterImpl = indexmap::map::IntoIter<String, Value>;
#[cfg(not(feature = "preserve_order"))]
type KeysImpl<'a> = btree_map::Keys<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type KeysImpl<'a> = indexmap::map::Keys<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type ValuesImpl<'a> = btree_map::Values<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type ValuesImpl<'a> = indexmap::map::Values<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type ValuesMutImpl<'a> = btree_map::ValuesMut<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type ValuesMutImpl<'a> = indexmap::map::ValuesMut<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type IntoValuesImpl = btree_map::IntoValues<String, Value>;
#[cfg(feature = "preserve_order")]
type IntoValuesImpl = indexmap::map::IntoValues<String, Value>;
use crate::error::Error;
use crate::value::Value;
use alloc::string::String;
#[cfg(feature = "preserve_order")]
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};
use core::iter::FusedIterator;
#[cfg(feature = "preserve_order")]
use core::mem;
use core::ops;
use serde::de;
#[cfg(not(feature = "preserve_order"))]
use alloc::collections::{btree_map, BTreeMap};
#[cfg(feature = "preserve_order")]
use indexmap::IndexMap;
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Value {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(null);
    /// ```
    Null,
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Bool(bool),
    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(12.5);
    /// ```
    Number(Number),
    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!("a string");
    /// ```
    String(String),
    /// Represents a JSON array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(["an", "array"]);
    /// ```
    Array(Vec<Value>),
    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "an": "object" });
    /// ```
    Object(Map<String, Value>),
}
impl Hash for Map<String, Value> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        #[cfg(not(feature = "preserve_order"))]
        {
            self.map.hash(state);
        }
        #[cfg(feature = "preserve_order")]
        {
            let mut kv = Vec::from_iter(&self.map);
            kv.sort_unstable_by(|a, b| a.0.cmp(b.0));
            kv.hash(state);
        }
    }
}
