use auth::auth_service;
use database::models::usuarios_models::UsuarioForm;
use database::repository::user_repository::{AuthRequest, AuthResponse};
use rocket::http::Status;
use rocket::serde::json::{self, Json};
use serde::Serialize;




#[post("/login", format = "json", data = "<auth_request>")]
// Request guard to ensure the request is valid
pub fn login(auth_request: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    let auth_response = auth_service::authenticate_user(auth_request.into_inner());

    match auth_response {
        Ok(auth_response) => Ok(Json(auth_response)),
        Err(e) => Err(e),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub fn register(user: Json<UsuarioForm>) -> Result<Json<AuthResponse>, Status> {
    let user_response = auth_service::register_user(user.into_inner());

    match user_response {
        Ok(user_response) => Ok(Json(user_response)),
        Err(e) => Err(e),
    }
}



