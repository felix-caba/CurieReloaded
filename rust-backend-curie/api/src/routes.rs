

use rocket::serde::json::Json;
use auth::auth_service;
use database::repository::user_repository::{AuthRequest, AuthResponse};

#[post("/login", format = "json", data = "<auth_request>")]
// Request guard to ensure the request is valid
pub fn login(auth_request: Json<AuthRequest>) -> Option<Json<AuthResponse>> {

    let auth_response = auth_service::authenticate_user(auth_request.into_inner());

    match auth_response {
        Ok(auth_response) => Some(Json(auth_response)),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

use database::models::NewUsuario;
#[post("/register", format = "json", data = "<user>")]
pub fn register(user: Json<NewUsuario>) -> Option<Json<AuthResponse>> {

    let user_response = auth_service::register_user(user.into_inner());

    match user_response {
        Ok(user_response) => Some(Json(user_response)),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}




