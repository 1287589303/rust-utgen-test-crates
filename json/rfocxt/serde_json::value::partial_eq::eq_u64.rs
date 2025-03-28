use super::Value;
use alloc::string::String;
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
impl Value {
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {}
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {}
    pub fn is_object(&self) -> bool {}
    pub fn as_object(&self) -> Option<&Map<String, Value>> {}
    pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {}
    pub fn is_array(&self) -> bool {}
    pub fn as_array(&self) -> Option<&Vec<Value>> {}
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {}
    pub fn is_string(&self) -> bool {}
    pub fn as_str(&self) -> Option<&str> {}
    pub fn is_number(&self) -> bool {}
    pub fn as_number(&self) -> Option<&Number> {}
    pub fn is_i64(&self) -> bool {}
    pub fn is_u64(&self) -> bool {}
    pub fn is_f64(&self) -> bool {}
    pub fn as_i64(&self) -> Option<i64> {}
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => n.as_u64(),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {}
    pub fn is_boolean(&self) -> bool {}
    pub fn as_bool(&self) -> Option<bool> {}
    pub fn is_null(&self) -> bool {}
    pub fn as_null(&self) -> Option<()> {}
    pub fn pointer(&self, pointer: &str) -> Option<&Value> {}
    pub fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Value> {}
    pub fn take(&mut self) -> Value {}
    pub fn sort_all_objects(&mut self) {}
}
fn eq_u64(value: &Value, other: u64) -> bool {
    value.as_u64() == Some(other)
}
