

use database::models::reactivos_models::{Reactivo, ReactivoForm};
use database::repository::productos_repository::{self};
use database::models::productos_models::{ProductoFormWithDetails, ProductoWithDetails};
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

use database::models::usuarios_models::UsuarioForm;


#[post("/register", format = "json", data = "<user>")]
pub fn register(user: Json<UsuarioForm>) -> Result<Json<AuthResponse>, Status> {
    let user_response = auth_service::register_user(user.into_inner());

    match user_response {
        Ok(user_response) =>  Ok(Json(user_response)),
        Err(e) =>  Err(e)
    }
}

#[get("/reactivos")]
pub fn get_reactivos() -> Result<Json<Vec<ProductoWithDetails<Reactivo>>>, Status> {
    let reactivos = productos_repository::select_reactivos();

    match reactivos {
        Ok(reactivos) => Ok(Json(reactivos)),
        Err(e) => {
            println!("[ProductoService] Error al obtener los reactivos: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/reactivos", format = "json", data = "<producto_form>")]
pub fn create_reactivo(producto_form: Json<ProductoFormWithDetails<ReactivoForm>>) -> Result<Json<ProductoWithDetails<Reactivo>>, Status> {
    let reactivo = productos_repository::insert_reactivo(producto_form.into_inner());

    match reactivo {
        Ok(reactivo) => Ok(Json(reactivo)),
        Err(e) => {
            println!("[ProductoService] Error al crear el reactivo: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/reactivos/<id>", format = "json", data = "<producto_form>")]
pub fn update_reactivo(id: i32, producto_form: Json<ProductoFormWithDetails<ReactivoForm>>) -> Result<Json<ProductoWithDetails<Reactivo>>, Status> {
    let reactivo = productos_repository::update_reactivo(id, producto_form.into_inner());

    match reactivo {
        Ok(reactivo) => Ok(Json(reactivo)),
        Err(e) => {
            println!("[ProductoService] Error al actualizar el reactivo: {}", e);
            Err(Status::InternalServerError)
        }
    }
}




