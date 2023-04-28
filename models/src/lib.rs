extern crate serde;
extern crate serde_json;

pub use serde::{Serialize, Deserialize};
pub use serde_json::{Result as JsonResult, Error as JsonError};

pub fn deserialize<'a, T: Deserialize<'a>>(s: &'a str) -> JsonResult<T> {
    serde_json::from_str(s)
}

pub fn serialize<T: Serialize>(data: &T) -> JsonResult<String> {
    serde_json::to_string(data)
}
