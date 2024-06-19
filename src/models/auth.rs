use serde::{Deserialize, Serialize};

use crate::models::base::default_empty_string;

// Orignally named: 'Body_admin_token_api_admin_token_post' in the openapi.json
#[derive(Serialize, Deserialize)]
pub struct BodyAdminTokenApiAdminTokenPost {
    grant_type: Option<String>,
    username: String,
    password: String,
    #[serde(default = "default_empty_string")]
    scope: String,
    client_id: Option<String>,
    client_secret: Option<String>,
}
