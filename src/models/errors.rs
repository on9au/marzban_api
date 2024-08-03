use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HTTPValidationError {
    detail: Option<Vec<ValidationError>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidationError {
    pub loc: Vec<ValidationErrorLocation>, // Location
    pub msg: String,                       // Message
    pub r#type: String,                    // Error Type
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidationErrorLocation {}
