use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HTTPValidationError {
    detail: Option<ValidationError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidationError {
    loc: ValidationErrorLocation, // Location
    msg: String,                  // Message
    r#type: String,               // Error Type
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ValidationErrorLocation {
    String(String),
    Integer(u32),
}
