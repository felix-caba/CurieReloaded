use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub token: String,
    pub success: bool,
}
