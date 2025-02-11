
use auth::jwt_service::JWT;
use database::models::localizaciones_models::{Localizacion, Ubicacion};
use database::repository::localizaciones_repository;
use rocket::http::Status;
use rocket::serde::json::Json;






#[get("/locations/select")]
pub fn get_locations(key: Result<JWT, Status>) -> Result<Json<Vec<Localizacion>>, Status> {

    key?;

    let locations = localizaciones_repository::select_locations();

    match locations {
        Ok(locations) => Ok(Json(locations)),
        Err(_) => Err(Status::InternalServerError),
    }   
}

#[get("/ubications/<id>")]
pub fn get_ubications_inside_location(id: i32, key: Result<JWT, Status>) -> Result<Json<Vec<Ubicacion>>, Status> {

    key?;

    let ubications = localizaciones_repository::view_ubications_of_location(id);
    match ubications {
        Ok(ubications) => Ok(Json(ubications)),
        Err(_) => Err(Status::InternalServerError),
    }

}