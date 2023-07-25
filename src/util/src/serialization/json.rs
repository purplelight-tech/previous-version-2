//! Working with JSON serialization.
//!
//! # Untyped JSON
//!
//! Work with untyped JSON using the [`json!`] macro and its main `Value` enumeration.
//!
//! ```
//! use rialight::util::serialization::json::json;
//! 
//! fn main() {
//!     let _ = json!({
//!         "x": "y"
//!     });
//! }
//! ```

use super::{Deserialize, Serialize};

pub use serde_json::{
    Deserializer,
    Error,
    Map,
    Number,
    Serializer,
    StreamDeserializer,
    Value,
    json
};
pub type Result<T> = std::result::Result<T, Error>;

/// Deserializes a JSON string into a value.
pub fn deserialize<'a, T>(string: &'a str) -> Result<T>
    where T: Deserialize<'a>
{
    serde_json::from_str(string)
}

/// Deserializes JSON given as a sequence of bytes into a value.
pub fn deserialize_from_slice<'a, T>(slice: &'a [u8]) -> Result<T>
    where T: Deserialize<'a>
{
    serde_json::from_slice(slice)
}

/// Deserializes JSON from a reader into a value.
pub fn deserialize_from_reader<R, T>(reader: R) -> Result<T>
    where
        R: std::io::Read,
        T: super::generic_deserialization::DeserializeOwned
{
    serde_json::from_reader(reader)
}

/// Interprets a `Value`, or untyped JSON data, as an instance of type `T`.
pub fn untyped_to_typed<T>(value: Value) -> Result<T>
    where T: super::generic_deserialization::DeserializeOwned
{
    serde_json::from_value(value)
}

/// Converts `T` into untyped JSON data of type `Value`.
pub fn typed_to_untyped<T>(value: T) -> Result<Value>
    where T: super::Serialize
{
    serde_json::to_value(value)
}

/// Serializes a value into a JSON string.
pub fn serialize<T>(value: &T) -> Result<String>
    where T: ?Sized + Serialize
{
    serde_json::to_string(value)
}

/// Serializes a value into a pretty-printed JSON string.
pub fn serialize_pretty<T>(value: &T) -> Result<String>
    where T: ?Sized + Serialize
{
    serde_json::to_string_pretty(value)
}

/// Serializes a value into JSON as a byte vector.
pub fn serialize_as_byte_vec<T>(value: &T) -> Result<Vec<u8>>
    where T: ?Sized + Serialize
{
    serde_json::to_vec(value)
}

/// Serializes a value into pretty-printed JSON as a byte vector.
pub fn serialize_as_byte_vec_pretty<T>(value: &T) -> Result<Vec<u8>>
    where T: ?Sized + Serialize
{
    serde_json::to_vec_pretty(value)
}

/// Serializes a value into JSON using an I/O stream.
pub fn serialize_with_writer<W, T>(writer: W, value: &T) -> Result<()>
    where
        W: std::io::Write,
        T: ?Sized + Serialize
{
    serde_json::to_writer(writer, value)
}

/// Serializes a value into pretty-printed JSON using an I/O stream.
pub fn serialize_with_writer_pretty<W, T>(writer: W, value: &T) -> Result<()>
    where
        W: std::io::Write,
        T: ?Sized + Serialize
{
    serde_json::to_writer_pretty(writer, value)
}

/// Work with untyped JSON values.
///
/// # Constructing JSON
///
/// The [`rialight::util::serialization::json::json!`] macro can be used to build
/// a [`Value`] with very natural JSON syntax.
///
/// ```
/// use rialight::util::serialization::json::json;
///
/// fn main() {
///     // The type of `jessica` is `Value`
///     let jessica = json!({
///         "name": "Jessica Clara",
///         "age": 16,
///         "phones": [
///             "+44 1234567",
///             "+44 2345678"
///         ]
///     });
///
///     println!("first phone number: {}", jessica["phones"][0]);
///
///     // Convert to a string of JSON and print it out
///     println!("{}", jessica.to_string());
/// }
/// ```
///
/// The `Value::to_string()` function converts a `Value` into a String of JSON text.
///
/// One neat thing about the `json!` macro is that variables and expressions
/// can be interpolated directly into the JSON value as you are building it.
/// At compile time it is checked that the value you are interpolating is able
/// to be represented as JSON.
///
/// ```
/// let full_name = "Jessica Clara";
/// let age_last_year = 16;
/// 
/// // The type of `jessica` is `Value`
/// let jessica = json!({
///     "name": full_name,
///     "age": age_last_year + 1,
///     "phones": [
///         format!("+44 {}", random_phone())
///     ]
/// });
/// ```
/// 
pub mod untyped_value {
    pub use serde_json::Value;
}