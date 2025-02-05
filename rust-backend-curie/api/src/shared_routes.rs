
use database::repository::shared;
use rocket::http::Status;
use rocket::serde::json::Json;

#[delete("/<id>")]
pub fn delete_producto(id: i32) -> Result<Json<()>, Status> {
    let result = shared::delete_producto(id);
    match result {
        Ok(()) => Ok(Json(())),
        Err(diesel::result::Error::NotFound) => Err(Status::NotFound),
        Err(e) => {
            println!("Error interno: {}", e);
            Err(Status::InternalServerError)
        }
    }
}