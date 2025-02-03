

use database::repository::productos_repository::{self, ProductoReactivo};
use rocket::http::Status;
use rocket::serde::json::Json;
use auth::auth_service;
use database::repository::user_repository::{AuthRequest, AuthResponse};



#[post("/login", format = "json", data = "<auth_request>")]
// Request guard to ensure the request is valid
pub fn login(auth_request: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    let auth_response = auth_service::authenticate_user(auth_request.into_inner());

    match auth_response {
        Ok(auth_response) => Ok(Json(auth_response)),
        Err(e) => Err(e)
    }
}

use database::models::NewUsuario;
#[post("/register", format = "json", data = "<user>")]
pub fn register(user: Json<NewUsuario>) -> Result<Json<AuthResponse>, Status> {
    let user_response = auth_service::register_user(user.into_inner());

    match user_response {
        Ok(user_response) =>  Ok(Json(user_response)),
        Err(e) =>  Err(e)
    }
}

#[get("/reactivos", format = "json")]
pub fn get_reactivos() -> Result<Json<Vec<ProductoReactivo>>, Status> {
    let reactivos = productos_repository::get_reactivos();

    match reactivos {
        Ok(reactivos) => Ok(Json(reactivos)),
        Err(e) => Err(e)
    }
}



