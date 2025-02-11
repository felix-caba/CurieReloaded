use auth::auth_service;
use auth::jwt_service::JWT;
use database::models::usuarios_models::{Usuario, UsuarioForm};
use database::repository::user_repository::{self, AuthRequest, AuthResponse};
use rocket::http::Status;
use rocket::serde::json::Json;




#[post("/login", format = "json", data = "<auth_request>")]
// Request guard to ensure the request is valid
pub fn login(auth_request: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    let auth_response = auth_service::authenticate_user(auth_request.into_inner());

    match auth_response {
        Ok(auth_response) => Ok(Json(auth_response)),
        Err(_) => Err(Status::Unauthorized),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub fn register(user: Json<UsuarioForm>) -> Result<Json<AuthResponse>, Status> {
    let user_response = auth_service::register_user(user.into_inner());

    match user_response {
        Ok(user_response) => Ok(Json(user_response)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/me")]
pub fn get_me(key: Result<JWT, Status>) -> Result<Json<Usuario>, Status> {
    let _key = key.map_err(|_| Status::Unauthorized)?;
    let user = user_repository::select_user_by_id(_key.claims.sub);
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(Status::Unauthorized),
    }
}



