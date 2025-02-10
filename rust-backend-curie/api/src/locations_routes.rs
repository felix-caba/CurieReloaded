use auth::jwt_service::JWT;
use database::models::localizaciones_models::Localizacion;
use database::repository::localizaciones_repository;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/locations/select")]
pub fn get_locations(key: Result<JWT, Status>) -> Result<Json<Vec<Localizacion>>, Status> {

    let _key = key?;

    // check if user is an admin:

    
    

    let locations = localizaciones_repository::select_locations();

    match locations {
        Ok(locations) => Ok(Json(locations)),
        Err(e) => Err(Status::InternalServerError),
    }   
}

#[get("/locations/select/<id>")]
pub fn get_location_by_id(id: i32) -> Result<Json<Localizacion>, Status> {
    let location = localizaciones_repository::select_location_by_id(id);

    match location {
        Ok(location) => Ok(Json(location)),
        Err(e) => Err(Status::InternalServerError),
    }
}


