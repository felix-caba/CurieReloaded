use auth::jwt_service::JWT;
use database::models::productos_models::{ProductoModel, ProductoModelForm};
use database::models::reactivos_models::{Reactivo, ReactivoForm};
use database::repository::productos_repository;
use rocket::http::Status;
use rocket::serde::json::Json;


#[get("/reactivo/select")]
pub fn get_reactivos(key: Result<JWT, Status>) -> Result<Json<Vec<ProductoModel<Reactivo>>>, Status> {
    
    key?;

    let reactivos = productos_repository::select_reactivos();

    match reactivos {
        Ok(reactivos) => Ok(Json(reactivos)),
        Err(e) => {
            println!("[ProductoService] Error al obtener los reactivos: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Request Guard, Reactivo form is valid
// Also Jwt implements fromRequest, but also Result<JWT, Status>
// only if T implements FromRequest & also e implements a responder
#[post("/reactivo/insert", format = "json", data = "<reactivo_form>")]
pub fn create_reactivo(
reactivo_form: Json<ProductoModelForm<ReactivoForm>>,
key: Result<JWT, Status>
) -> Result<Json<ProductoModel<Reactivo>>, Status> {

    key?;

    let reactivo = productos_repository::insert_reactivo(reactivo_form.into_inner());

    match reactivo {
        Ok(producto) => Ok(Json(producto)),
        Err(e) => {
            println!("[ProductoService] Error al crear el reactivo: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/reactivo/update/<id>", format = "json", data = "<reactivo_form>")]
pub fn update_reactivo(
    id: i32,
    reactivo_form: Json<ProductoModelForm<ReactivoForm>>,
) -> Result<Json<ProductoModel<Reactivo>>, Status> {
    let reactivo = productos_repository::update_reactivo(reactivo_form.into_inner(), id);

    match reactivo {
        Ok(reactivo) => Ok(Json(reactivo)),
        Err(e) => {
            println!("[ProductoService] Error al actualizar el reactivo: {}", e);
            Err(Status::InternalServerError)
        }
    }
}
