extern crate serde;
extern crate serde_json;

pub use serde::{Deserialize, Serialize};
pub use serde_json::{Error as JsonError, Result as JsonResult};

pub fn deserialize<'a, T: Deserialize<'a>>(s: &'a str) -> JsonResult<T> {
    serde_json::from_str(s)
}

pub fn serialize<T: Serialize>(data: &T) -> JsonResult<String> {
    serde_json::to_string(data)
}
