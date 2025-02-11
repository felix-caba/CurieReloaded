#[macro_use]
extern crate rocket;

mod reactivos_routes;
mod shared_routes;
mod usuarios_routes;
mod locations_routes;
mod catcher;

use locations_routes::*;
use reactivos_routes::*;
use shared_routes::*;
use usuarios_routes::*;
use catcher::*;


#[rocket::main]
pub async fn launch_api() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/api/auth", routes![login, register, get_me])
        .mount(
            "/api",
            routes![create_reactivo, update_reactivo, get_reactivos],
        )
        .mount("/api/locations", routes![get_locations, get_ubications_inside_location])
        .mount("/api/shared", routes![delete_producto])
        .register("/", catchers![not_found, unauthorized, forbidden, internal_server_error])
        .launch()
        .await?;

    Ok(())
}
